//! Currency type with IRS-compliant rounding rules.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents a USD currency amount with cent precision.
///
/// Internally uses `rust_decimal::Decimal` to avoid floating-point errors.
/// All arithmetic preserves exact values; rounding is explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UsdAmount(Decimal);

impl UsdAmount {
    /// Zero dollars.
    pub const ZERO: UsdAmount = UsdAmount(dec!(0));

    /// Create from a decimal value.
    pub fn new(amount: Decimal) -> Self {
        Self(amount)
    }

    /// Create from dollars and cents.
    ///
    /// # Example
    /// ```
    /// use honest_tax_core::UsdAmount;
    /// let amount = UsdAmount::from_cents(12345); // $123.45
    /// ```
    pub fn from_cents(cents: i64) -> Self {
        Self(Decimal::new(cents, 2))
    }

    /// Create from whole dollars.
    pub fn from_dollars(dollars: i64) -> Self {
        Self(Decimal::new(dollars, 0))
    }

    /// Returns the inner decimal value.
    pub fn as_decimal(&self) -> Decimal {
        self.0
    }

    /// Returns the amount in cents (truncated).
    pub fn as_cents(&self) -> i64 {
        (self.0 * dec!(100)).trunc().to_i64().unwrap_or(0)
    }

    /// Returns true if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns true if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative() && !self.0.is_zero()
    }

    /// Returns true if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Returns the minimum of two amounts.
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns the maximum of two amounts.
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Round to the nearest dollar (IRS standard for most calculations).
    ///
    /// Per IRS instructions: "Round off cents to whole dollars."
    /// Amounts under 50 cents round down; 50 cents and over round up.
    pub fn round_to_dollar(&self) -> Self {
        Self(self.0.round_dp_with_strategy(
            0,
            rust_decimal::RoundingStrategy::MidpointAwayFromZero,
        ))
    }

    /// Truncate to whole dollars (floor for positive, ceiling for negative).
    pub fn trunc_to_dollar(&self) -> Self {
        Self(self.0.trunc())
    }

    /// Round to cents (2 decimal places).
    pub fn round_to_cents(&self) -> Self {
        Self(self.0.round_dp_with_strategy(
            2,
            rust_decimal::RoundingStrategy::MidpointAwayFromZero,
        ))
    }

    /// Multiply by a decimal rate (e.g., tax rate).
    pub fn multiply_rate(&self, rate: Decimal) -> Self {
        Self(self.0 * rate)
    }

    /// Saturating subtraction: returns zero if result would be negative.
    pub fn saturating_sub(&self, other: Self) -> Self {
        if self.0 >= other.0 {
            Self(self.0 - other.0)
        } else {
            Self::ZERO
        }
    }
}

impl Default for UsdAmount {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for UsdAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

impl Add for UsdAmount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for UsdAmount {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for UsdAmount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for UsdAmount {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl core::iter::Sum for UsdAmount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl From<Decimal> for UsdAmount {
    fn from(d: Decimal) -> Self {
        Self(d)
    }
}

impl From<UsdAmount> for Decimal {
    fn from(m: UsdAmount) -> Self {
        m.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cents() {
        let m = UsdAmount::from_cents(12345);
        assert_eq!(m.as_cents(), 12345);
        assert_eq!(m.to_string(), "$123.45");
    }

    #[test]
    fn test_from_dollars() {
        let m = UsdAmount::from_dollars(100);
        assert_eq!(m.as_cents(), 10000);
    }

    #[test]
    fn test_rounding() {
        // $123.49 rounds to $123
        let m1 = UsdAmount::from_cents(12349);
        assert_eq!(m1.round_to_dollar().as_cents(), 12300);

        // $123.50 rounds to $124
        let m2 = UsdAmount::from_cents(12350);
        assert_eq!(m2.round_to_dollar().as_cents(), 12400);

        // $123.51 rounds to $124
        let m3 = UsdAmount::from_cents(12351);
        assert_eq!(m3.round_to_dollar().as_cents(), 12400);
    }

    #[test]
    fn test_saturating_sub() {
        let a = UsdAmount::from_dollars(100);
        let b = UsdAmount::from_dollars(150);
        assert_eq!(a.saturating_sub(b), UsdAmount::ZERO);
        assert_eq!(b.saturating_sub(a), UsdAmount::from_dollars(50));
    }

    #[test]
    fn test_saturating_sub_equal() {
        let a = UsdAmount::from_dollars(100);
        assert_eq!(a.saturating_sub(a), UsdAmount::ZERO);
    }

    #[test]
    fn test_sum() {
        let amounts = vec![
            UsdAmount::from_dollars(100),
            UsdAmount::from_dollars(200),
            UsdAmount::from_dollars(300),
        ];
        let total: UsdAmount = amounts.into_iter().sum();
        assert_eq!(total, UsdAmount::from_dollars(600));
    }

    #[test]
    fn test_negative_amounts() {
        let neg = UsdAmount::from_dollars(-50);
        let pos = UsdAmount::from_dollars(50);
        let zero = UsdAmount::ZERO;

        assert!(neg.is_negative());
        assert!(!neg.is_positive());
        assert!(!neg.is_zero());

        assert!(pos.is_positive());
        assert!(!pos.is_negative());
        assert!(!pos.is_zero());

        assert!(zero.is_zero());
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
    }

    #[test]
    fn test_abs() {
        let neg = UsdAmount::from_dollars(-50);
        let pos = UsdAmount::from_dollars(50);
        assert_eq!(neg.abs(), pos);
        assert_eq!(pos.abs(), pos);
    }

    #[test]
    fn test_rounding_negative() {
        // -$123.49 rounds to -$123
        let m1 = UsdAmount::from_cents(-12349);
        assert_eq!(m1.round_to_dollar(), UsdAmount::from_dollars(-123));

        // -$123.50 rounds to -$124
        let m2 = UsdAmount::from_cents(-12350);
        assert_eq!(m2.round_to_dollar(), UsdAmount::from_dollars(-124));
    }

    #[test]
    fn test_trunc_to_dollar() {
        // Positive: truncates toward zero
        let m1 = UsdAmount::from_cents(12399);
        assert_eq!(m1.trunc_to_dollar(), UsdAmount::from_dollars(123));

        // Negative: truncates toward zero (ceiling)
        let m2 = UsdAmount::from_cents(-12399);
        assert_eq!(m2.trunc_to_dollar(), UsdAmount::from_dollars(-123));
    }

    #[test]
    fn test_round_to_cents() {
        let m = UsdAmount::new(Decimal::new(123456, 3)); // 123.456
        assert_eq!(m.round_to_cents(), UsdAmount::from_cents(12346));

        let m2 = UsdAmount::new(Decimal::new(123454, 3)); // 123.454
        assert_eq!(m2.round_to_cents(), UsdAmount::from_cents(12345));

        let m3 = UsdAmount::new(Decimal::new(123455, 3)); // 123.455
        assert_eq!(m3.round_to_cents(), UsdAmount::from_cents(12346));
    }

    #[test]
    fn test_multiply_rate() {
        let income = UsdAmount::from_dollars(50000);
        let rate = dec!(0.22); // 22% tax rate
        let tax = income.multiply_rate(rate);
        assert_eq!(tax, UsdAmount::from_dollars(11000));

        // Zero rate
        assert_eq!(income.multiply_rate(dec!(0)), UsdAmount::ZERO);

        // Rate > 1
        assert_eq!(
            UsdAmount::from_dollars(100).multiply_rate(dec!(1.5)),
            UsdAmount::from_dollars(150)
        );

        // Negative rate
        assert_eq!(
            UsdAmount::from_dollars(100).multiply_rate(dec!(-0.1)),
            UsdAmount::from_dollars(-10)
        );
    }

    #[test]
    fn test_min_max() {
        let a = UsdAmount::from_dollars(100);
        let b = UsdAmount::from_dollars(200);

        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);
        assert_eq!(b.min(a), a);
        assert_eq!(b.max(a), b);

        // Same values
        assert_eq!(a.min(a), a);
        assert_eq!(a.max(a), a);
    }

    #[test]
    fn test_subtraction() {
        let a = UsdAmount::from_dollars(200);
        let b = UsdAmount::from_dollars(50);

        assert_eq!(a - b, UsdAmount::from_dollars(150));
        assert_eq!(b - a, UsdAmount::from_dollars(-150));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = UsdAmount::from_dollars(200);
        a -= UsdAmount::from_dollars(50);
        assert_eq!(a, UsdAmount::from_dollars(150));
    }
}
