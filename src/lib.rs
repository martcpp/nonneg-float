//! A generic wrapper for non-negative floating point values.
//!
//! Ensures that values are >= 0 and finite, providing safe construction
//! methods and a convenient macro.
//!
//! Supports any float type implementing `num_traits::Float`.
//!
//! # Examples
//!
//! ```
//! use nonneg_float::{NonNegative, nonneg};
//!
//! let zero = NonNegative::<f64>::zero();
//! let val = NonNegative::try_new(3.14).unwrap();
//! let macro_val = nonneg!(5.0f64).unwrap();
//!
//! assert_eq!(zero.get(), 0.0);
//! assert_eq!(val.get(), 3.14);
//! assert_eq!(macro_val.get(), 5.0);
//! ```

use num_traits::Float;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error type returned when trying to create a `NonNegative` from an invalid value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonNegativeError {
    /// The value was negative or not finite.
    InvalidValue,
}

impl fmt::Display for NonNegativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NonNegativeError::InvalidValue => write!(f, "Value must be non-negative and finite"),
        }
    }
}

impl std::error::Error for NonNegativeError {}

/// Wrapper type guaranteeing a non-negative floating-point value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct NonNegative<T: Float>(T);

impl<T: Float> NonNegative<T> {
    /// Returns a `NonNegative` wrapping zero.
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self(T::zero())
    }

    /// Attempts to create a new `NonNegative<T>` from a value.
    ///
    /// Returns `Err` if the value is negative or not finite.
    pub fn try_new(value: T) -> Result<Self, NonNegativeError> {
        if value >= T::zero() && value.is_finite() {
            Ok(Self(value))
        } else {
            Err(NonNegativeError::InvalidValue)
        }
    }

    /// Creates a new `NonNegative<T>` or panics if invalid.
    ///
    /// # Panics
    ///
    /// Panics if the value is negative or not finite.
    pub fn new(value: T) -> Self {
        Self::try_new(value).expect("Value must be non-negative and finite")
    }

    /// Returns the inner float value.
    pub fn get(&self) -> T {
        self.0
    }
}

impl<T: Float + num_traits::Zero> Default for NonNegative<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Float + fmt::Display> fmt::Display for NonNegative<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Macro to create a `NonNegative` value.
///
/// Returns `Result<NonNegative<T>, NonNegativeError>`.
///
/// Usage:
/// - `nonneg!(value)` infers type and creates a `NonNegative` from `value`.
/// - `nonneg!(Type)` creates a default zero value of that type.
/// - `nonneg!(Type, value)` creates a `NonNegative` of the specified type.
#[macro_export]
macro_rules! nonneg {
    ($t:ty) => {
        $crate::NonNegative::<$t>::zero()
    };
    ($val:expr) => {{ $crate::NonNegative::try_new($val) }};
    ($t:ty, $val:expr) => {{ $crate::NonNegative::<$t>::try_new($val) }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let zero = NonNegative::<f64>::zero();
        assert_eq!(zero.get(), 0.0);
        let default: NonNegative<f64> = Default::default();
        assert_eq!(default.get(), 0.0);
    }

    #[test]
    fn test_try_new_valid() {
        let val = NonNegative::try_new(3.14f64).unwrap();
        assert_eq!(val.get(), 3.14);
    }

    #[test]
    fn test_try_new_invalid() {
        assert_eq!(
            NonNegative::try_new(-0.1f64).unwrap_err(),
            NonNegativeError::InvalidValue
        );
        assert_eq!(
            NonNegative::try_new(f64::NAN).unwrap_err(),
            NonNegativeError::InvalidValue
        );
        assert_eq!(
            NonNegative::try_new(f64::INFINITY).unwrap_err(),
            NonNegativeError::InvalidValue
        );
    }

    #[test]
    fn test_new_panics() {
        let _ = NonNegative::new(1.0f64); // okay
    }

    #[test]
    #[should_panic(expected = "Value must be non-negative and finite")]
    fn test_new_panics_on_invalid() {
        let _ = NonNegative::new(-1.0f64);
    }

    #[test]
    fn test_macro() {
        let a = nonneg!(5.0f64).unwrap();
        assert_eq!(a.get(), 5.0);

        let b = nonneg!(f64);
        assert_eq!(b.get(), 0.0);

        let c = nonneg!(f32, 2.71).unwrap();
        assert_eq!(c.get(), 2.71);

        let d = nonneg!(-1.0f64);
        assert!(d.is_err());
    }
}
