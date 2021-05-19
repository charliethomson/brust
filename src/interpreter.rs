use std::{
    collections::HashMap,
    fs,
    io::{Read, Stdin, Stdout, StdoutLock, Write},
    path::Path,
};

use crate::{ast::*, Parser};

type BuiltinFunction = fn(Vec<Expression>) -> Expression;

#[derive(Debug, Clone)]
pub struct Function {
    args: Vec<Identifier>,
    body: Statement,
    builtin: Option<BuiltinFunction>,
}
impl Function {
    fn Builtin() -> Self {
        Self {
            args: vec![],
            body: Statement::Null,
            builtin: None,
        }
    }

    pub fn Puts() -> Self {
        let mut func = Self::Builtin();

        func.builtin = Some(|args| {
            args.iter()
                .for_each(|expr| println!("{}", expr.expect_const().unwrap()));
            return Expression::Constant(Const::Integer(0));
        });

        func
    }

    pub fn Format() -> Self {
        let mut func = Self::Builtin();

        func.builtin = Some(|args| {
            if let Const::String(s) = args.get(0).unwrap().expect_const().unwrap() {
                let mut s = s;
                let mut args = match args.get(1).map(|a| a.expect_const()).flatten() {
                    Some(Const::Vector(mut v)) => v
                        .into_iter()
                        .map(|boxed| Box::leak(boxed).clone().expect_const().unwrap())
                        .collect::<Vec<Const>>(),
                    _ => panic!("second argument of format must be a vector constant"),
                };

                while (s.contains("{}")) {
                    s = s.replacen("{}", args.remove(0).to_string().as_str(), 1);
                }

                return Expression::Constant(Const::String(s));
            }
            panic!("First argument of format must be a string constant");
        });

        func
    }
}

#[derive(Debug)]
pub struct Scope {
    functions: HashMap<Identifier, Function>,
    variables: HashMap<Identifier, Option<Const>>,
    extern_variables: HashMap<Identifier, Option<Const>>,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
            extern_variables: HashMap::new(),
        }
    }

    pub fn global() -> Self {
        let mut functions = HashMap::new();
        functions.insert(Identifier::Name("puts".into()), Function::Puts());
        functions.insert(Identifier::Name("format".into()), Function::Format());

        let mut variables = HashMap::new();
        Self {
            functions,
            variables,
            extern_variables: HashMap::new(),
        }
    }
    fn has_var(&self, ident: &Identifier) -> bool {
        self.variables.contains_key(ident)
    }

    fn set_var(&mut self, ident: &Identifier, value: Option<Const>) {
        self.variables.insert(ident.clone(), value);
    }
    fn get_var(&self, ident: &Identifier) -> Option<Const> {
        self.variables.get(ident).cloned().flatten()
    }
    fn get_func(&self, ident: &Identifier) -> Option<&Function> {
        self.functions.get(ident)
    }
    fn add_func(&mut self, ident: Identifier, function: Function) {
        self.functions.insert(ident, function);
    }
}

#[derive(Debug)]
pub struct Interpreter {
    buffer: String,
    // 0 is global
    scopes: Vec<Scope>,
    // stdout: Stdout,
    // // stdin: Stdin,
}
impl Interpreter {
    // fn put<S: ToString>(&mut self, s: S) -> std::io::Result<()> {
    //     self.stdout.write(s.to_string().as_bytes())?;
    //     self.stdout.flush()?;
    //     Ok(())
    // }

    // // i32 is probably overkill lol
    // fn scope_depth(s: &String) -> i32 {
    //     s.chars()
    //         .filter(|ch| "{}".contains(*ch))
    //         .fold(0, |depth, current_char| {
    //             if current_char == '{' {
    //                 depth + 1
    //             } else {
    //                 depth - 1
    //             }
    //         })
    // }

    // fn get_stmt(&mut self) -> std::io::Result<String> {
    //     let mut buffer = String::new();

    //     self.put(">  ");
    //     self.stdin.read_line(&mut buffer)?;

    //     while (Self::scope_depth(&buffer) != 0) {
    //         let indent = (0..Self::scope_depth(&buffer) + 1)
    //             .map(|_| ">  ")
    //             .collect::<String>();
    //         self.put(indent);
    //         self.stdin.read_line(&mut buffer)?;
    //     }

    //     Ok(buffer)
    // }
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            scopes: vec![Scope::global()],
            // // // stdout: std::io::stdout(),
            // // // stdin: std::io::stdin(),
        }
    }

    pub fn add_var(&mut self, ident: &Identifier, value: Option<&Expression>) {
        let value = match value {
            Some(Expression::Constant(c)) => Some(c.clone()),
            Some(e) => match self.eval_expr(e.clone()) {
                Expression::Constant(c) => Some(c),
                _ => panic!("Cannot set variable to a non value expression"),
            },
            None => None,
        };

        self.current_scope().set_var(ident, value.clone());
    }

    //TODO
    pub fn add_extern(&mut self, ident: &Identifier) {}

    pub fn call_main(&mut self) -> Const {
        self.call_function(&Identifier::Name("main".into()), vec![])
            .expect_const()
            .unwrap()
    }

    pub fn call_function(
        &mut self,
        ident: &Identifier,
        arguments: Vec<Box<Expression>>,
    ) -> Expression {
        let func = self.get_func(ident).expect("Function not defined");
        if let Some(builtin) = func.builtin {
            return builtin(
                arguments
                    .into_iter()
                    .map(|boxed| self.eval_expr(Box::leak(boxed).clone()))
                    .collect(),
            );
        }

        // Create a new scope for the function,
        let mut scope = Scope::new();
        for (ident, arg) in func.args.iter().zip(arguments.into_iter()) {
            scope.set_var(ident, self.eval_expr(Box::leak(arg).clone()).expect_const());
        }
        self.scopes.push(scope);

        // Execute the function in that scope

        let result = self
            .eval_stmt(func.body.clone())
            .unwrap_or(Expression::Constant(Const::Integer(0)));

        self.scopes.pop();
        result
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
    pub fn global_scope(&mut self) -> &mut Scope {
        self.scopes.get_mut(0).unwrap()
    }

    pub fn get_var(&mut self, ident: &Identifier) -> Option<Expression> {
        self.current_scope()
            .get_var(ident)
            .or_else(|| self.global_scope().get_var(ident))
            .map(|constant| Expression::Constant(constant))
    }

    pub fn get_func(&mut self, ident: &Identifier) -> Option<Function> {
        self.global_scope().get_func(ident).cloned()
    }

    fn binary_operation(
        &mut self,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        operation_function: fn(&Const, &Const) -> Const,
    ) -> Expression {
        let lhs = self
            .eval_expr(Box::leak(lhs).clone())
            .expect_const()
            .unwrap_or_else(|| panic!("Binary operation lhs non const value"));

        let rhs = self
            .eval_expr(Box::leak(rhs).clone())
            .expect_const()
            .unwrap_or_else(|| panic!("Binary operation rhs non const value"));

        Expression::Constant(operation_function(&lhs, &rhs))
    }

    fn unary_operation(
        &mut self,
        rhs: Box<Expression>,
        operation_function: fn(&Const) -> Const,
    ) -> Expression {
        let rhs = self
            .eval_expr(Box::leak(rhs).clone())
            .expect_const()
            .unwrap_or_else(|| panic!("Cannot operate on non const value"));

        Expression::Constant(operation_function(&rhs))
    }

    fn comparison(
        &mut self,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        comparison_function: fn(&Expression, &Expression) -> bool,
    ) -> Expression {
        if comparison_function(
            &self.eval_expr(Box::leak(lhs).clone()),
            &self.eval_expr(Box::leak(rhs).clone()),
        ) {
            Expression::Constant(Const::Integer(1))
        } else {
            Expression::Constant(Const::Integer(0))
        }
    }

    fn assign_and(
        &mut self,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        constructor: fn(Box<Expression>, Box<Expression>) -> Expression,
    ) -> Expression {
        match Box::leak(lhs) {
            Expression::Identifier(ident) => {
                let value = self.eval_expr(Box::leak(rhs).clone());
                let current_value = self
                    .get_var(ident)
                    .expect(&format!("'{}' is not defined", ident));
                let value = self.eval_expr(constructor(
                    Box::new(current_value.clone()),
                    Box::new(value.clone()),
                ));

                self.add_var(ident, Some(&value));
                return value;
            }
            _ => panic!("Cannot assign to non ident"),
        }
    }

    fn conditional_expr(&mut self, condition: Box<Expression>) -> bool {
        self.eval_expr(Box::leak(condition).clone())
            .expect_const()
            .map(|c| c.truthy())
            .unwrap_or(/* TODO: FAIL STATE GOES HERE */ false)
    }

    fn get_const(&mut self, ident: &Identifier) -> Const {
        self.get_var(ident)
            .unwrap_or_else(|| panic!("{} not defined!", ident))
            .expect_const()
            .unwrap_or_else(|| panic!("attempted to get the const value of a non value identifier {} (this should be impossible)", ident))
    }

    pub fn eval_expr(&mut self, expr: Expression) -> Expression {
        match expr {
            Expression::Assign { lhs, rhs } => match Box::leak(lhs) {
                Expression::Identifier(ident) => {
                    let value = self.eval_expr(Box::leak(rhs).clone());
                    self.add_var(ident, Some(&value));
                    return value;
                }
                _ => panic!("Cannot assign to non ident"),
            },
            Expression::AssignOr { lhs, rhs } => self.assign_and(lhs, rhs, Expression::or),
            Expression::AssignXor { lhs, rhs } => self.assign_and(lhs, rhs, Expression::xor),
            Expression::AssignAnd { lhs, rhs } => self.assign_and(lhs, rhs, Expression::and),
            Expression::AssignShiftLeft { lhs, rhs } => {
                self.assign_and(lhs, rhs, Expression::shift_left)
            }
            Expression::AssignShiftRight { lhs, rhs } => {
                self.assign_and(lhs, rhs, Expression::shift_right)
            }
            Expression::AssignAdd { lhs, rhs } => self.assign_and(lhs, rhs, Expression::add),
            Expression::AssignSubtract { lhs, rhs } => {
                self.assign_and(lhs, rhs, Expression::subtract)
            }
            Expression::AssignMultiply { lhs, rhs } => {
                self.assign_and(lhs, rhs, Expression::multiply)
            }
            Expression::AssignDivide { lhs, rhs } => self.assign_and(lhs, rhs, Expression::divide),
            Expression::AssignModulo { lhs, rhs } => self.assign_and(lhs, rhs, Expression::modulo),
            Expression::Equal { lhs, rhs } => self.comparison(lhs, rhs, Expression::eq),
            Expression::NotEqual { lhs, rhs } => self.comparison(lhs, rhs, Expression::ne),
            Expression::Less { lhs, rhs } => self.comparison(lhs, rhs, Expression::lt),
            Expression::More { lhs, rhs } => self.comparison(lhs, rhs, Expression::gt),
            Expression::LessEqual { lhs, rhs } => self.comparison(lhs, rhs, Expression::le),
            Expression::MoreEqual { lhs, rhs } => self.comparison(lhs, rhs, Expression::ge),
            Expression::Or { lhs, rhs } => self.binary_operation(lhs, rhs, Const::or),
            Expression::Xor { lhs, rhs } => self.binary_operation(lhs, rhs, Const::xor),
            Expression::And { lhs, rhs } => self.binary_operation(lhs, rhs, Const::and),
            Expression::ShiftLeft { lhs, rhs } => self.binary_operation(lhs, rhs, Const::shl),
            Expression::ShiftRight { lhs, rhs } => self.binary_operation(lhs, rhs, Const::shr),
            Expression::Add { lhs, rhs } => self.binary_operation(lhs, rhs, Const::add),
            Expression::Subtract { lhs, rhs } => self.binary_operation(lhs, rhs, Const::sub),
            Expression::Multiply { lhs, rhs } => self.binary_operation(lhs, rhs, Const::mul),
            Expression::Divide { lhs, rhs } => self.binary_operation(lhs, rhs, Const::div),
            Expression::Modulo { lhs, rhs } => self.binary_operation(lhs, rhs, Const::modulo),
            Expression::Not { rhs } => self.unary_operation(rhs, Const::not),
            Expression::Complement { rhs } => self.unary_operation(rhs, Const::complement),
            Expression::UnaryPlus { rhs } => self.eval_expr(Box::leak(rhs).clone()),
            Expression::UnaryMinus { rhs } => self.unary_operation(rhs, Const::negate),
            Expression::Ternary { condition, yes, no } => {
                if self.conditional_expr(condition) {
                    self.eval_expr(Box::leak(yes).clone())
                } else {
                    self.eval_expr(Box::leak(no).clone())
                }
            }
            Expression::PreIncrement { rhs } => match Box::leak(rhs) {
                Expression::Identifier(ident) => {
                    let value = Expression::Constant(self.get_const(ident).inc());
                    self.add_var(ident, Some(&value));

                    value
                }
                Expression::Constant(c) => Expression::Constant(c.inc()),
                _ => panic!("Cannot increment non identifier/constant value"),
            },
            Expression::PreDecrement { rhs } => match Box::leak(rhs) {
                Expression::Identifier(ident) => {
                    let value = Expression::Constant(self.get_const(ident).dec());
                    self.add_var(ident, Some(&value));

                    value
                }
                Expression::Constant(c) => Expression::Constant(c.dec()),
                _ => panic!("Cannot decrement non identifier/constant value"),
            },
            Expression::PostIncrement { lhs } => match Box::leak(lhs) {
                Expression::Identifier(ident) => {
                    let value = self.get_const(ident);
                    self.add_var(ident, Some(&Expression::Constant(value.inc())));

                    Expression::Constant(value)
                }
                Expression::Constant(c) => Expression::Constant(c.inc()),
                _ => panic!("Cannot increment non identifier/constant value"),
            },
            Expression::PostDecrement { lhs } => match Box::leak(lhs) {
                Expression::Identifier(ident) => {
                    let value = self.get_const(ident);
                    self.add_var(ident, Some(&Expression::Constant(value.dec())));

                    Expression::Constant(value)
                }
                Expression::Constant(c) => Expression::Constant(c.dec()),
                _ => panic!("Cannot decrement non identifier/constant value"),
            },
            Expression::VectorIndex { vector, index } => {
                let vector = self
                    .eval_expr(Box::leak(vector).clone())
                    .expect_const()
                    .unwrap_or_else(|| panic!("Cannot index non vector object"));
                let index = self
                    .eval_expr(Box::leak(index).clone())
                    .expect_const()
                    .unwrap_or_else(|| panic!("Cannot index vector with non constant value"));
                Box::leak(vector.index(index)).clone()
            }
            Expression::Constant(v) => match v {
                Const::Vector(v) => Expression::Constant(Const::Vector(
                    v.into_iter()
                        .map(|expr| self.eval_expr(Box::leak(expr).clone()))
                        .map(Box::new)
                        .collect(),
                )),
                _ => Expression::Constant(v),
            },
            Expression::Identifier(i) => self
                .get_var(&i)
                .unwrap_or_else(|| panic!("{} is not defined", i)),
            Expression::FunctionCall { ident, args } => self.call_function(&ident, args),
        }
    }

    pub fn eval_stmt(&mut self, stmt: Statement) -> Option<Expression> {
        match stmt {
            Statement::Compound(stmts) => {
                for stmt in stmts.into_iter() {
                    if let Some(return_value) = self.eval_stmt(stmt) {
                        return Some(return_value);
                    }
                }
                None
            }
            Statement::Return(expr) => Some(self.eval_expr(expr)),
            Statement::Expression(expr) => {
                self.eval_expr(expr);
                None
            }

            Statement::Declaration { scope, idents } => {
                match scope {
                    VariableScope::Extern => idents.iter().for_each(|ident| self.add_extern(ident)),

                    VariableScope::Local => {
                        idents.iter().for_each(|ident| self.add_var(ident, None))
                    }
                };
                None
            }
            Statement::Conditional { condition, body, e } => {
                if self.conditional_expr(Box::new(condition)) {
                    self.eval_stmt(Box::leak(body).clone())
                } else if let Some(e) = e {
                    self.eval_stmt(Box::leak(e).clone())
                } else {
                    None
                }
            }
            Statement::Loop { condition, body } => {
                let body = Box::leak(body).clone();
                let condition = Box::new(condition);
                while self.conditional_expr(condition.clone()) {
                    if let Some(return_value) = self.eval_stmt(body.clone()) {
                        return Some(return_value);
                    }
                }
                None
            }
            Statement::Switch {
                switching_on,
                cases,
            } => None,
            // TODO: Figure out how to do this lol
            Statement::Label(ident) => unimplemented!(),
            Statement::Goto(ident) => unimplemented!(),

            Statement::FunctionDefinition { ident, args, body } => {
                self.global_scope().add_func(
                    ident,
                    Function {
                        args,
                        body: Box::leak(body).clone(),
                        builtin: None,
                    },
                );
                None
            }
            Statement::GlobalDefinition {
                ident,
                initial_value,
            } => None,

            Statement::Break => None,
            Statement::Null => None,
        }
    }

    pub fn eval(&mut self, file: Vec<Statement>) {
        for stmt in file.into_iter() {
            self.eval_stmt(stmt);
        }
    }

    pub fn interpret<P: AsRef<Path>>(mut self, path: P) -> Result<(), String> {
        let mut buffer = fs::read_to_string(path).unwrap();
        let mut ast = Parser::new().parse(&buffer).unwrap();
        //
        // let s = r#"
        // main() {
        //     auto a;
        //     a = 10;
        //     return sub5(a);
        // }
        //
        // sub5(a) {
        //     return a - 5;
        // }
        // "#;
        // let e = crate::grammar::FileParser::new().parse(s).unwrap();
        println!("Evaluating: {}", buffer);
        // println!("Expr: {:#?}", ast);
        self.eval(ast);

        println!(
            "Result: {:?}",
            self.call_function(&Identifier::Name("main".into()), vec![])
                .expect_const()
                .unwrap()
        );

        Ok(())
    }

    pub fn interpret_string<S: ToString>(mut self, s: S) -> i64 {
        let ast = Parser::new().parse(&s.to_string()).unwrap();
        self.eval(ast);

        let result = self.call_main();

        match result {
            Const::Integer(i) => i,
            other => {
                println!("Result: {:?}", other);
                return 0;
            }
        }
    }
}
