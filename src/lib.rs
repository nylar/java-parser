#[macro_use]
extern crate nom;

mod parser;

use parser::compilation_unit;

#[derive(Debug, PartialEq, Eq)]
pub struct CompilationUnit {
    pub package: Option<String>,
    pub imports: Vec<Import>,
    // TODO: Top level declarations, Class, Enum, Interface, etc.
    pub classes: Vec<Class>,
    pub annotations: Vec<Annotation>,
}

impl CompilationUnit {
    pub fn new() -> Self {
        CompilationUnit {
            package: None,
            imports: Vec::new(),
            classes: Vec::new(),
            annotations: Vec::new(),
        }
    }

    pub fn parse<S: AsRef<[u8]>>(file: S) -> Result<Self, ::nom::IError> {
        let file = file.as_ref();

        match compilation_unit(file) {
            ::nom::IResult::Done(_, r) => Ok(r),
            o => o.to_full_result(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub path: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub access_modifier: Option<AccessModifier>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub access_modifier: Option<AccessModifier>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldType {
    String,
    Boolean,
    Long,
    Int,
    Short,
    Type(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccessModifier {
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Method {
    pub name: String,
    pub return_type: FieldType,
    pub arguments: String,
    pub access_modifier: Option<AccessModifier>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Annotation {
    pub name: String,
    pub options: String,
}
