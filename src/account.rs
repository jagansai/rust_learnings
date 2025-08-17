use crate::person::Person;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountType {
    Normal,
    Premium,
    Other(String),
}

impl AccountType {
    pub fn as_lowercase_key(&self) -> String {
        match self {
            AccountType::Normal => "normal".to_string(),
            AccountType::Premium => "premium".to_string(),
            AccountType::Other(s) => s.trim().to_lowercase(),
        }
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::Normal => write!(f, "Normal"),
            AccountType::Premium => write!(f, "Premium"),
            AccountType::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "normal" => Ok(AccountType::Normal),
            "premium" => Ok(AccountType::Premium),
            other => Ok(AccountType::Other(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub owner: Person,
    pub account_type: AccountType,
    pub balance: f64,
    pub account_number: String,
}

impl Account {
    pub fn new(owner: Person, account_type: AccountType, balance: f64, account_number: String) -> Self {
        Self { owner, account_type, balance, account_number }
    }
}
