use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}` cannot be turned into a class")]
    BadClassInput(String),
    #[error("`{0}` cannot be turned into an ancestry")]
    BadAncestryInput(String),
}
