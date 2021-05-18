use std::{
    fmt::{Display, Formatter},
    ops::BitAnd,
};

#[derive(Debug, Clone)]
pub enum VariableScope {
    Extern,
    Local,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd)]
pub enum Identifier {
    Name(String),
    Vector(String, i64),
}
impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Name(n) => n,
                Self::Vector(n, _) => n,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Const {
    Integer(i64),
    String(String),
    Vector(Vec<Box<Expression>>),
    Ident(Identifier),
}
impl Const {
    pub fn truthy(&self) -> bool {
        match self {
            Self::Integer(i) => *i != 0,
            Self::String(s) => s.len() != 0,
            Self::Vector(v) => v.len() != 0,
            _ => true,
        }
    }

    pub fn and(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs & rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() && rhs.truthy()) as i64),
        }
    }
    pub fn or(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs | rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() || rhs.truthy()) as i64),
        }
    }
    pub fn xor(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs ^ rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() ^ rhs.truthy()) as i64),
        }
    }
    pub fn shr(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs >> rhs),
            (lhs, rhs) => Self::Integer(lhs.truthy() as i64 >> rhs.truthy() as i64),
        }
    }
    pub fn shl(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs << rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() as i64) << (rhs.truthy() as i64)),
        }
    }
    pub fn add(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs + rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() as i64) + (rhs.truthy() as i64)),
        }
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs - rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() as i64) - (rhs.truthy() as i64)),
        }
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs * rhs),
            (lhs, _) => Self::Integer(lhs.truthy() as i64),
        }
    }
    pub fn div(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs / rhs),
            (lhs, _) => Self::Integer(lhs.truthy() as i64),
        }
    }
    pub fn modulo(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs % rhs),
            (lhs, rhs) => Self::Integer((lhs.truthy() as i64) % (rhs.truthy() as i64)),
        }
    }
    pub fn complement(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(!i),
            i => Self::Integer(!i.truthy() as i64),
        }
    }
    pub fn negate(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(-i),
            i => Self::Integer(-(i.truthy() as i64)),
        }
    }
    pub fn inc(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(i + 1),
            i => Self::Integer(i.truthy() as i64 + 1),
        }
    }
    pub fn dec(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(i - 1),
            i => Self::Integer(i.truthy() as i64 - 1),
        }
    }
    pub fn index(&self, idx: Const) -> Box<Expression> {
        match (self, idx) {
            (Self::Vector(v), Self::Integer(i)) => {
                v.get(i as usize).expect("Index out of bounds").clone()
            }
            (Self::Vector(_), _) => panic!("Attempt to index with non integer index"),
            _ => panic!("Attempt to index a non vector object"),
        }
    }
}
impl Display for Const {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s.trim_matches('"')),
            _ => write!(f, ""),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
impl Expression {
    pub fn expect_const(&self) -> Option<Const> {
        match self {
            Self::Constant(c) => Some(c.clone()),
            _ => None,
        }
    }
    pub fn expect_ident(&self) -> Option<Identifier> {
        match self {
            Self::Identifier(i) => Some(i.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CaseStatement {
    pub case: Const,
    pub body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone)]
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
