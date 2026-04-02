//! Kernel-specific error types.

use tapthe_ai_types::error::TaptheAiError;
use thiserror::Error;

/// Kernel error type wrapping TaptheAiError with kernel-specific context.
#[derive(Error, Debug)]
pub enum KernelError {
    /// A wrapped TaptheAiError.
    #[error(transparent)]
    Tapthe.ai(#[from] TaptheAiError),

    /// The kernel failed to boot.
    #[error("Boot failed: {0}")]
    BootFailed(String),
}

/// Alias for kernel results.
pub type KernelResult<T> = Result<T, KernelError>;
