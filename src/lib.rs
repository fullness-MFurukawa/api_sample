pub mod handler;
pub mod jwt;
pub mod error;

use crate::error::ApiAppError;
pub type Result<T> = anyhow::Result<T, ApiAppError>;
