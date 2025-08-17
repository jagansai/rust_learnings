
use crate::account::{Account, AccountType};
use crate::validators::Validator;

pub struct NormalDiscountValidator {}

impl Validator for NormalDiscountValidator {
    fn validate(&self, account: &Account) -> Result<(), String> {
        if account.account_type != AccountType::Normal {
            return Err(format!("Account '{}' is not a Normal account", account.account_number));
        }
        if account.balance < 100.0 {
            return Err(format!("Normal account '{}' has insufficient balance for discount: {}", account.account_number, account.balance));
        }
        Ok(())
    }
    fn name(&self) -> &'static str {
        "NormalDiscountValidator"
    }
}
