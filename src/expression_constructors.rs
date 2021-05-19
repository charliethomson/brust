use crate::ast::{Const, Expression, Identifier};

impl Expression {
    pub fn assign(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Assign { lhs, rhs }
    }
    pub fn assign_or(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignOr { lhs, rhs }
    }
    pub fn assign_xor(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignXor { lhs, rhs }
    }
    pub fn assign_and(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignAnd { lhs, rhs }
    }
    pub fn assign_shift_left(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignShiftLeft { lhs, rhs }
    }
    pub fn assign_shift_right(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignShiftRight { lhs, rhs }
    }
    pub fn assign_add(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignAdd { lhs, rhs }
    }
    pub fn assign_subtract(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignSubtract { lhs, rhs }
    }
    pub fn assign_multiply(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignMultiply { lhs, rhs }
    }
    pub fn assign_divide(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignDivide { lhs, rhs }
    }
    pub fn assign_modulo(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::AssignModulo { lhs, rhs }
    }
    pub fn ternary(condition: Box<Expression>, yes: Box<Expression>, no: Box<Expression>) -> Self {
        Self::Ternary { condition, yes, no }
    }
    pub fn equal(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Equal { lhs, rhs }
    }
    pub fn not_equal(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::NotEqual { lhs, rhs }
    }
    pub fn less(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Less { lhs, rhs }
    }
    pub fn more(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::More { lhs, rhs }
    }
    pub fn less_equal(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::LessEqual { lhs, rhs }
    }
    pub fn more_equal(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::MoreEqual { lhs, rhs }
    }
    pub fn or(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Or { lhs, rhs }
    }
    pub fn xor(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Xor { lhs, rhs }
    }
    pub fn and(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::And { lhs, rhs }
    }
    pub fn shift_left(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::ShiftLeft { lhs, rhs }
    }
    pub fn shift_right(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::ShiftRight { lhs, rhs }
    }
    pub fn add(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Add { lhs, rhs }
    }
    pub fn subtract(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Subtract { lhs, rhs }
    }
    pub fn multiply(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Multiply { lhs, rhs }
    }
    pub fn divide(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Divide { lhs, rhs }
    }
    pub fn modulo(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        Self::Modulo { lhs, rhs }
    }
    pub fn not(rhs: Box<Expression>) -> Self {
        Self::Not { rhs }
    }
    pub fn complement(rhs: Box<Expression>) -> Self {
        Self::Complement { rhs }
    }
    pub fn pre_increment(rhs: Box<Expression>) -> Self {
        Self::PreIncrement { rhs }
    }
    pub fn pre_decrement(rhs: Box<Expression>) -> Self {
        Self::PreDecrement { rhs }
    }
    pub fn unary_plus(rhs: Box<Expression>) -> Self {
        Self::UnaryPlus { rhs }
    }
    pub fn unary_minus(rhs: Box<Expression>) -> Self {
        Self::UnaryMinus { rhs }
    }
    pub fn post_increment(lhs: Box<Expression>) -> Self {
        Self::PostIncrement { lhs }
    }
    pub fn post_decrement(lhs: Box<Expression>) -> Self {
        Self::PostDecrement { lhs }
    }
    pub fn vector_index(vector: Box<Expression>, index: Box<Expression>) -> Self {
        Self::VectorIndex { vector, index }
    }
    pub fn constant(item: Const) -> Self {
        Self::Constant(item)
    }
    pub fn identifier(item: Identifier) -> Self {
        Self::Identifier(item)
    }
    pub fn function_call(ident: Identifier, args: Vec<Box<Expression>>) -> Self {
        Self::FunctionCall { ident, args }
    }
}
