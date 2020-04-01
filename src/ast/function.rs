use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub code: Vec<Statement>,
}
