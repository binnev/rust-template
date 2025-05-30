#[doc = include_str!("../docs/ref/add.md")]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[doc = include_str!("../docs/ref/subtract.md")]
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[doc = include_str!("../docs/ref/multiply.md")]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// You can also put your docstring inline, but then you can't show it in the
/// mkdocs site.
///
/// Divides the first number by the second and returns the result.
///
/// # Panics
///
/// Panics if the second number is zero.
///
/// # Examples
///
/// ```
/// let result = rust_template::divide(6.0, 3.0);
/// assert_eq!(result, 2.0);
/// ```
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 3.0), 2.0);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(1.0, 0.0);
    }
}
