#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Decode(String),
    Convert(String),
    Unpack(String),
}
