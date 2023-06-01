use std::fmt;

#[derive(Clone, Debug)]
pub enum DataType {
    Null,
    Bool,
    Int,
    Long,
    Float,
    Double,
    Char,
    String,
    Array
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Null => write!(f, "null"),
            DataType::Bool => write!(f, "bool"),
            DataType::Int => write!(f, "int"),
            DataType::Long => write!(f, "long"),
            DataType::Float => write!(f, "float"),
            DataType::Double => write!(f, "double"),
            DataType::Char => write!(f, "char"),
            DataType::String => write!(f, "string"),
            DataType::Array => write!(f, "array"),
        }
    }
}