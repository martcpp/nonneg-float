use nonneg_float::{NonNegative, nonneg};

#[test]
fn test_valid_values() {
    let n = nonneg!(42.0f64);
    assert_eq!(n.unwrap().get(), 42.0);
}

#[test]
fn test_invalid_value() {
    let result = nonneg!(-1.0f64);
    assert!(result.is_err());
}

#[test]
fn test_default_value() {
    let d: NonNegative<f32> = Default::default();
    assert_eq!(d.get(), 0.0);
}
