use std::fmt;

#[derive(Clone, Debug)]
pub enum DataType {
    Null,
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Char,
    String,
    Array
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Null => write!(f, "null"),
            DataType::Bool => write!(f, "bool"),
            DataType::Int32 => write!(f, "int"),
            DataType::Int64 => write!(f, "long"),
            DataType::Float32 => write!(f, "float"),
            DataType::Float64 => write!(f, "double"),
            DataType::Char => write!(f, "char"),
            DataType::String => write!(f, "string"),
            DataType::Array => write!(f, "array"),
        }
    }
}