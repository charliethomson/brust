#[derive(Debug)]
pub enum VariableScope {
    Extern,
    Local,
}

#[derive(Debug)]
pub enum Identifier {
    Name(String),
    Vector(String, i64),
}

#[derive(Debug)]
pub enum Const {
    Integer(i64),
    String(String),
}

#[derive(Debug)]
pub enum Expression {
    Assign {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignOr {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignXor {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignAnd {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignShiftLeft {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignShiftRight {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignAdd {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignSubtract {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignMultiply {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignDivide {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    AssignModulo {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        yes: Box<Expression>,
        no: Box<Expression>,
    },
    Equal {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    NotEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Less {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    More {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    LessEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    MoreEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Or {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Xor {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    And {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    ShiftLeft {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    ShiftRight {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Subtract {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Multiply {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Divide {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Modulo {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Not {
        rhs: Box<Expression>,
    },
    Complement {
        rhs: Box<Expression>,
    },
    PreIncrement {
        rhs: Box<Expression>,
    },
    PreDecrement {
        rhs: Box<Expression>,
    },
    UnaryPlus {
        rhs: Box<Expression>,
    },
    UnaryMinus {
        rhs: Box<Expression>,
    },
    PostIncrement {
        lhs: Box<Expression>,
    },
    PostDecrement {
        lhs: Box<Expression>,
    },
    VectorIndex {
        vector: Box<Expression>,
        index: Box<Expression>,
    },
    Constant(Const),
    Identifier(Identifier),
    FunctionCall {
        ident: Identifier,
        args: Vec<Box<Expression>>,
    },
}

#[derive(Debug)]
pub struct CaseStatement {
    pub case: Const,
    pub body: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub enum Statement {
    Compound(Vec<Statement>),
    Return(Expression),
    Expression(Expression),
    Declaration {
        scope: VariableScope,
        idents: Vec<Identifier>,
    },
    Conditional {
        condition: Expression,
        body: Box<Statement>,
        e: Option<Box<Statement>>,
    },
    Loop {
        condition: Expression,
        body: Box<Statement>,
    },
    // TODO: Default
    Switch {
        switching_on: Expression,
        cases: Vec<CaseStatement>,
    },
    Label(Identifier),
    Goto(Identifier),
    FunctionDefinition {
        ident: Identifier,
        args: Vec<Identifier>,
        body: Box<Statement>,
    },
    GlobalDefinition {
        ident: Identifier,
        initial_value: Const,
    },

    Break,

    Null,
}
