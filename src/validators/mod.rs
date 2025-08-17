use crate::account::Account;

pub trait Validator {
    fn validate(&self, account: &Account) -> Result<(), String>;
    /// A friendly name for the validator used in logs and output.
    fn name(&self) -> &'static str;
}

pub mod normal_minimum_balance;
pub mod premium_minimum_balance;
pub mod normal_discount;
pub mod premium_discount;

// Utility to load validators by name. For this demo we wire a few names to concrete types.
pub fn get_validator_by_name(name: &str) -> Option<Box<dyn Validator>> {
    match name {
    "NormalBalanceValidator" => Some(Box::new(crate::validators::normal_minimum_balance::NormalMinimum {})),
    "PremiumBalanceValidator" => Some(Box::new(crate::validators::premium_minimum_balance::PremiumMinimum {})),
    // accept both singular and plural forms used in XML
    "NormalDiscountValidator" => Some(Box::new(crate::validators::normal_discount::NormalDiscountValidator {})),
    "PremiumDiscountValidator" => Some(Box::new(crate::validators::premium_discount::PremiumDiscountValidator {})),
        _ => None,
    }
}
