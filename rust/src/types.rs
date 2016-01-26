use std::vec;

pub enum MalType {
    Nil,
    Int(i64),
    MString(String),
    True,
    False,
    Sym(String),
    Vector(Vec<MalType>),
    Keyword(String),
}

impl MalType {
    pub fn typename(&self) -> String {
        match *self {
            MalType::Nil => "Nil".to_string(),
            MalType::Int(_) => "Int".to_string(),
            MalType::MString(_) => "MString".to_string(),
            MalType::True => "True".to_string(),
            MalType::False => "False".to_string(),
            MalType::Sym(_) => "Sym".to_string(),
            MalType::Vector(_) => "Vector".to_string(),
            MalType::Keyword(_) => "Keyword".to_string(),
        }
    }
}
