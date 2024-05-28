use thiserror::Error;

#[derive(Error, Debug)]
pub enum TgtgError{
    #[error("Polling failed with error `{0}`")]
    PollingError(String)
}