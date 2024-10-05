use thiserror::Error;

pub type MatsyaRelayResult<T> = Result<T, MatsyaRelayError>;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum MatsyaRelayError {
    #[error("Not implemented")]
    NotImplemented,
}
