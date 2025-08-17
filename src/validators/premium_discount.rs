
use crate::account::{Account, AccountType};
use crate::validators::Validator;

pub struct PremiumDiscountValidator {}

impl Validator for PremiumDiscountValidator {
    fn validate(&self, account: &Account) -> Result<(), String> {
        if account.account_type != AccountType::Premium {
            return Err(format!("Account '{}' is not a Premium account", account.account_number));
        }
        if account.balance < 500.0 {
            return Err(format!("Premium account '{}' has insufficient balance for discount: {}", account.account_number, account.balance));
        }
        Ok(())
    }
    fn name(&self) -> &'static str {
        "PremiumDiscountValidator"
    }
}