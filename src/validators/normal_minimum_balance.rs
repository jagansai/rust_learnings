use crate::account::Account;
use crate::validators::Validator;

pub struct NormalMinimum {}

impl Validator for NormalMinimum {
    fn validate(&self, account: &Account) -> Result<(), String> {
        if account.balance < 100.0 {
            Err(format!("Normal account '{}' has insufficient balance: {}", account.account_number, account.balance))
        } else {
            Ok(())
        }
    }
    fn name(&self) -> &'static str {
        "NormalMinimum"
    }
}
