/// Represents a value that can either be exact or a lossy (truncated) version due to irreversible data loss.
///
/// # Variants
/// - `Exact(T)`: Contains the exact original value without any loss.
/// - `Trunc(T)`: Contains a truncated value where some data has been irreversibly lost.
///
/// # Example
/// ```
/// fn lossy_op(count: u32) -> Lossy<u8> {
///     if count <= 255 {
///         Lossy::Exact(count as u8)
///     } else {
///         Lossy::Trunc(255) // Truncation due to overflow
///     }
/// }
///
/// // Usage:
/// let val = lossy_op(200);
/// assert_eq!(val.unwrap(), 200); // Returns exact value, no panic.
///
/// let truncated = lossy_op(404);
/// // truncated.unwrap(); // This line would panic because the value is truncated.
///
/// let accepted = truncated.ok(); // Explicitly accept the truncated value.
/// assert_eq!(accepted, 255);
/// ```
pub enum Lossy<T> {
    Exact(T),
    Trunc(T)
}

impl<A> Lossy<A> {
    pub fn anyhow(self) -> A {
        match self {
            Self::Exact(x) | Self::Trunc(x) => x
        }
    }

    pub fn ok(self) -> Option<A> {
        match self {
            Self::Exact(x) => Some(x),
            Self::Trunc(_) => None
        }
    }

    pub fn is_exact(&self) -> bool {
        if let Self::Exact(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_trunc(&self) -> bool {
        if let Self::Trunc(_) = self {
            true
        } else {
            false
        }
    }
}

impl<T> From<Lossy<T>> for Option<T> {
    fn from(value: Lossy<T>) -> Self {
        value.ok()
    }
}

