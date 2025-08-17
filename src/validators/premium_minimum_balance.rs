use crate::account::Account;
use crate::validators::Validator;

pub struct PremiumMinimum {}

impl Validator for PremiumMinimum {
    fn validate(&self, account: &Account) -> Result<(), String> {
        if account.balance < 1000.0 {
            Err(format!("Premium account '{}' requires balance >= 1000. Got {}", account.account_number, account.balance))
        } else {
            Ok(())
        }
    }
    fn name(&self) -> &'static str {
        "PremiumMinimum"
    }
}
