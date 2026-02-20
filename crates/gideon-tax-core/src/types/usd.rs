use core::fmt;
use core::iter::Sum;
use core::ops::{Add, Mul, Neg, Sub};

/// US dollar amount stored as whole cents for exact arithmetic.
///
/// All arithmetic is performed on the underlying `i64` cent value,
/// avoiding floating-point rounding errors common in financial calculations.
///
/// Overflow is not checked because `i64::MAX` cents ≈ $92 quadrillion,
/// a value no tax return will ever approach.
///
/// # Examples
///
/// ```
/// use gideon_tax_core::Usd;
///
/// let income = Usd::from_dollars(50_000);
/// let deduction = Usd::from_dollars(14_600);
/// let taxable = income - deduction;
/// assert_eq!(taxable, Usd::from_dollars(35_400));
///
/// let amount = Usd::from_cents(1_050);
/// assert_eq!(amount.irs_round(), Usd::from_dollars(11));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Usd(i64);

impl Usd {
    /// Zero dollars.
    pub const ZERO: Self = Usd(0);

    /// Creates a `Usd` value from a whole-dollar amount.
    pub const fn from_dollars(dollars: i64) -> Self {
        Usd(dollars * 100)
    }

    /// Creates a `Usd` value from a cent amount.
    pub const fn from_cents(cents: i64) -> Self {
        Usd(cents)
    }

    /// Returns the total value in cents.
    pub const fn cents(self) -> i64 {
        self.0
    }

    /// Rounds toward positive infinity to the nearest whole dollar.
    ///
    /// # Examples
    ///
    /// ```
    /// use gideon_tax_core::Usd;
    ///
    /// assert_eq!(Usd::from_cents(150).round_up(), Usd::from_dollars(2));
    /// assert_eq!(Usd::from_cents(100).round_up(), Usd::from_dollars(1));
    /// assert_eq!(Usd::from_cents(-150).round_up(), Usd::from_dollars(-1));
    /// ```
    pub const fn round_up(self) -> Self {
        let rem = self.0.rem_euclid(100);
        if rem == 0 {
            self
        } else {
            Usd(self.0 + (100 - rem))
        }
    }

    /// Rounds toward negative infinity to the nearest whole dollar.
    ///
    /// # Examples
    ///
    /// ```
    /// use gideon_tax_core::Usd;
    ///
    /// assert_eq!(Usd::from_cents(150).round_down(), Usd::from_dollars(1));
    /// assert_eq!(Usd::from_cents(100).round_down(), Usd::from_dollars(1));
    /// assert_eq!(Usd::from_cents(-150).round_down(), Usd::from_dollars(-2));
    /// ```
    pub const fn round_down(self) -> Self {
        let rem = self.0.rem_euclid(100);
        Usd(self.0 - rem)
    }

    /// Rounds to the nearest whole dollar using the IRS whole-dollar method.
    ///
    /// Amounts below 50 cents are dropped; amounts of 50 cents or more
    /// are rounded up to the next dollar. Rounding is applied to the absolute
    /// value, so negative amounts round away from zero.
    ///
    /// See: <https://www.irs.gov/instructions/i1040gi#en_US_2024_publink100080020>
    ///
    /// # Examples
    ///
    /// ```
    /// use gideon_tax_core::Usd;
    ///
    /// assert_eq!(Usd::from_cents(149).irs_round(), Usd::from_dollars(1));
    /// assert_eq!(Usd::from_cents(150).irs_round(), Usd::from_dollars(2));
    /// assert_eq!(Usd::from_cents(151).irs_round(), Usd::from_dollars(2));
    /// ```
    pub const fn irs_round(self) -> Self {
        let abs = self.0.unsigned_abs();
        let rem = abs % 100;
        let rounded = if rem >= 50 {
            abs + (100 - rem)
        } else {
            abs - rem
        };
        if self.0 < 0 {
            Usd(-(rounded as i64))
        } else {
            Usd(rounded as i64)
        }
    }
}

impl Add for Usd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Usd(self.0 + rhs.0)
    }
}

impl Sub for Usd {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Usd(self.0 - rhs.0)
    }
}

impl Neg for Usd {
    type Output = Self;

    fn neg(self) -> Self {
        Usd(-self.0)
    }
}

impl Mul<i64> for Usd {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Usd(self.0 * rhs)
    }
}

impl Sum for Usd {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Usd::ZERO, Add::add)
    }
}

impl fmt::Display for Usd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs = self.0.unsigned_abs();
        write!(f, "{}${}.{:02}", sign, abs / 100, abs % 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dollars() {
        assert_eq!(Usd::from_dollars(42).cents(), 4200);
        assert_eq!(Usd::from_dollars(-5).cents(), -500);
        assert_eq!(Usd::from_dollars(0).cents(), 0);
    }

    #[test]
    fn from_cents() {
        assert_eq!(Usd::from_cents(4200).cents(), 4200);
        assert_eq!(Usd::from_cents(-1).cents(), -1);
    }

    #[test]
    fn add() {
        let a = Usd::from_dollars(10);
        let b = Usd::from_cents(550);
        assert_eq!(a + b, Usd::from_cents(1550));
    }

    #[test]
    fn sub() {
        let a = Usd::from_dollars(10);
        let b = Usd::from_cents(550);
        assert_eq!(a - b, Usd::from_cents(450));
    }

    #[test]
    fn mul() {
        assert_eq!(Usd::from_dollars(5) * 3, Usd::from_dollars(15));
        assert_eq!(Usd::from_cents(33) * 2, Usd::from_cents(66));
    }

    #[test]
    fn round_up_positive() {
        assert_eq!(Usd::from_cents(100).round_up(), Usd::from_dollars(1));
        assert_eq!(Usd::from_cents(101).round_up(), Usd::from_dollars(2));
        assert_eq!(Usd::from_cents(199).round_up(), Usd::from_dollars(2));
        assert_eq!(Usd::from_cents(1).round_up(), Usd::from_dollars(1));
        assert_eq!(Usd::ZERO.round_up(), Usd::ZERO);
    }

    #[test]
    fn round_up_negative() {
        assert_eq!(Usd::from_cents(-100).round_up(), Usd::from_dollars(-1));
        assert_eq!(Usd::from_cents(-101).round_up(), Usd::from_dollars(-1));
        assert_eq!(Usd::from_cents(-199).round_up(), Usd::from_dollars(-1));
        assert_eq!(Usd::from_cents(-1).round_up(), Usd::ZERO);
    }

    #[test]
    fn round_down_positive() {
        assert_eq!(Usd::from_cents(100).round_down(), Usd::from_dollars(1));
        assert_eq!(Usd::from_cents(101).round_down(), Usd::from_dollars(1));
        assert_eq!(Usd::from_cents(199).round_down(), Usd::from_dollars(1));
        assert_eq!(Usd::from_cents(1).round_down(), Usd::ZERO);
        assert_eq!(Usd::ZERO.round_down(), Usd::ZERO);
    }

    #[test]
    fn round_down_negative() {
        assert_eq!(Usd::from_cents(-100).round_down(), Usd::from_dollars(-1));
        assert_eq!(Usd::from_cents(-101).round_down(), Usd::from_dollars(-2));
        assert_eq!(Usd::from_cents(-199).round_down(), Usd::from_dollars(-2));
        assert_eq!(Usd::from_cents(-1).round_down(), Usd::from_dollars(-1));
    }

    #[test]
    fn irs_round_positive() {
        assert_eq!(Usd::from_cents(149).irs_round(), Usd::from_dollars(1));
        assert_eq!(Usd::from_cents(150).irs_round(), Usd::from_dollars(2));
        assert_eq!(Usd::from_cents(151).irs_round(), Usd::from_dollars(2));
        assert_eq!(Usd::from_cents(100).irs_round(), Usd::from_dollars(1));
        assert_eq!(Usd::from_cents(199).irs_round(), Usd::from_dollars(2));
        assert_eq!(Usd::ZERO.irs_round(), Usd::ZERO);
    }

    #[test]
    fn irs_round_negative() {
        assert_eq!(Usd::from_cents(-149).irs_round(), Usd::from_dollars(-1));
        assert_eq!(Usd::from_cents(-150).irs_round(), Usd::from_dollars(-2));
        assert_eq!(Usd::from_cents(-151).irs_round(), Usd::from_dollars(-2));
        assert_eq!(Usd::from_cents(-100).irs_round(), Usd::from_dollars(-1));
        assert_eq!(Usd::from_cents(-199).irs_round(), Usd::from_dollars(-2));
    }

    #[test]
    fn display() {
        assert_eq!(Usd::from_cents(1050).to_string(), "$10.50");
        assert_eq!(Usd::from_cents(5).to_string(), "$0.05");
        assert_eq!(Usd::from_dollars(100).to_string(), "$100.00");
        assert_eq!(Usd::from_cents(-1050).to_string(), "-$10.50");
        assert_eq!(Usd::ZERO.to_string(), "$0.00");
    }

    #[test]
    fn ordering() {
        assert!(Usd::from_dollars(10) > Usd::from_dollars(5));
        assert!(Usd::from_cents(-1) < Usd::ZERO);
    }

    #[test]
    fn neg() {
        assert_eq!(-Usd::from_dollars(5), Usd::from_dollars(-5));
        assert_eq!(-Usd::from_dollars(-3), Usd::from_dollars(3));
        assert_eq!(-Usd::ZERO, Usd::ZERO);
    }

    #[test]
    fn sum() {
        let amounts = vec![
            Usd::from_dollars(100),
            Usd::from_dollars(200),
            Usd::from_cents(50),
        ];
        assert_eq!(amounts.into_iter().sum::<Usd>(), Usd::from_cents(30_050));
    }

    #[test]
    fn sum_empty() {
        let amounts: Vec<Usd> = vec![];
        assert_eq!(amounts.into_iter().sum::<Usd>(), Usd::ZERO);
    }

    #[test]
    fn default_is_zero() {
        assert_eq!(Usd::default(), Usd::ZERO);
    }
}
