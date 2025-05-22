//! A generic NonNegative float wrapper with a macro for safe construction.
//!
//! Ensures the float value is non-negative at runtime and defaults to zero if no value is provided.

use num_traits::Float;

/// Wrapper type for non-negative floats.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct NonNegative<T: Float>(T);

impl<T: Float> NonNegative<T> {
    /// Creates a new `NonNegative<T>` if the value is >= 0 and finite.
    pub fn new(value: T) -> Option<Self> {
        if value >= T::zero() && value.is_finite() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the inner value.
    pub fn get(&self) -> T {
        self.0
    }
}

/// Macro for easy creation of `NonNegative` values.
///
/// Usage:
/// - `nonneg!(Type)` creates `NonNegative<Type>` with default 0.
/// - `nonneg!(value)` creates with inferred type, panics if negative.
/// - `nonneg!(Type, value)` creates with explicit type and value.
///
/// Panics at runtime if value is negative.
///
/// # Examples
///
/// ```
/// use nonneg_float::{NonNegative, nonneg};
///
/// let a = nonneg!(f64);
/// let b = nonneg!(5.5f64);
/// let c = nonneg!(f32, 3.14);
///
/// assert_eq!(a.get(), 0.0);
/// assert_eq!(b.get(), 5.5);
/// assert_eq!(c.get(), 3.14);
/// ```
#[macro_export]
macro_rules! nonneg {
    ($t:ty) => {
        $crate::NonNegative::<$t>::new(0.0 as $t).unwrap()
    };

    ($val:expr) => {{
        let val = $val;
        if val < 0.0 {
            panic!("Value must be non-negative");
        }
        $crate::NonNegative::new(val).unwrap()
    }};

    ($t:ty, $val:expr) => {{
        let val: $t = $val;
        if val < 0.0 {
            panic!("Value must be non-negative");
        }
        $crate::NonNegative::<$t>::new(val).unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        assert_eq!(NonNegative::new(5.0f64).unwrap().get(), 5.0);
        assert_eq!(NonNegative::new(0.0f32).unwrap().get(), 0.0);
    }

    #[test]
    fn test_new_invalid() {
        assert!(NonNegative::new(-1.0f64).is_none());
        assert!(NonNegative::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_macro_default() {
        let x = nonneg!(f64);
        assert_eq!(x.get(), 0.0);
    }

    #[test]
    fn test_macro_value() {
        let x = nonneg!(7.5f64);
        assert_eq!(x.get(), 7.5);
    }

    #[test]
    #[should_panic(expected = "Value must be non-negative")]
    fn test_macro_negative_panics() {
        let _ = nonneg!(-2.0);
    }

    #[test]
    fn test_macro_type_value() {
        let x = nonneg!(f32, 3.14);
        assert_eq!(x.get(), 3.14);
    }
}
