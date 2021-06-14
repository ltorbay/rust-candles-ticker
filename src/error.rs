#[derive(Debug)]
pub struct Error<T> {
    pub kind: Kind,
    pub cause: Option<T>
}

#[derive(Debug)]
pub enum Kind {
    Client
}