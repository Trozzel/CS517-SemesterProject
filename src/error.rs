use thiserror::Error;

pub type MyErr = Box<dyn std::error::Error + Sync + Send + 'static>;
pub type MyRes<T> = Result<T, MyErr>;

#[derive(Debug, Error)]
pub enum TMFromStrError {
    #[error("Data should be arranged in multiples of {col}")]
    InvalidDimensions{col: usize}
}
