#[derive(Debug, Clone)]
pub struct JavaFile {
    pub name: String,
    pub raw_content: String,
}

#[derive(Debug, Clone)]
pub struct RustFile {
    pub name: String,
    pub translated_content: String,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Class {
        name: String,
        methods: Vec<AstNode>,
    },
    Method {
        name: String,
        return_type: String,
        params: Vec<(String, String)>,
        body: String,
    },
    Field {
        name: String,
        field_type: String,
    },
}

pub trait JavaParser {
    fn parse(&self, file: &JavaFile) -> Result<Vec<AstNode>, String>;
}
