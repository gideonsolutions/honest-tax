/// IRS filing status for Form 1040.
///
/// Filing status determines tax rates, standard deduction amounts, and eligibility
/// for certain credits and deductions.
///
/// See: <https://www.irs.gov/publications/p501#en_US_2024_publink1000220721>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilingStatus {
    /// Unmarried or legally separated/divorced on the last day of the tax year,
    /// and not qualifying for another filing status.
    Single,

    /// Married couples who agree to file a joint return, combining their income,
    /// deductions, and credits. Both spouses are jointly liable for the tax.
    MarriedFilingJointly,

    /// Married individuals who choose to file separate returns. May be beneficial
    /// when one spouse has significant medical expenses or miscellaneous deductions.
    MarriedFilingSeparately,

    /// Unmarried individuals who paid more than half the cost of keeping up a home
    /// for a qualifying person (such as a dependent child or parent).
    HeadOfHousehold,

    /// A surviving spouse whose spouse died during one of the two prior tax years
    /// and who has a dependent child. Allows use of joint return tax rates.
    QualifyingSurvivingSpouse,
}
