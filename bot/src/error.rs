use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("registering game failed")]
    RegisterFailed,
}
