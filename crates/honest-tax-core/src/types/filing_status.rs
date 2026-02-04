/// IRS filing status for Form 1040
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilingStatus {
    Single,
    MarriedFilingJointly,
    MarriedFilingSeparately,
    HeadOfHousehold,
    QualifyingSurvivingSpouse,
}
