use super::statement::Statement;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub code: Vec<Statement>,
}
