pub enum RegisterError {
    TableLookupError,
}

pub enum DeserializationError {
    FailedToDeserialize,
}

pub enum CompilerError {}

pub enum EditorError {
    Null,
    InvalidCodeRoot(String),
    UnknownTableRefrence(String),
}
