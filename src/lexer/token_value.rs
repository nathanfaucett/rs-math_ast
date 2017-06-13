use collections::string::String;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenValue {
    Str(String),
    Chr(char),
}
