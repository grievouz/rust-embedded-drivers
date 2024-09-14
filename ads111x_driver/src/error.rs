use thiserror::Error;

#[derive(Error, Debug)]
pub enum ADSError<E> {
    #[error("Failed to convert config")]
    ConfigConversionError,
    #[error("Invalid I2C address")]
    WrongAddress,
    #[error("I2C communication error: {0}")]
    I2C(#[from] E),
}
