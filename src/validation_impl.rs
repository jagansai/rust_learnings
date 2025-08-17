use crate::account::{Account, AccountType};
use crate::person::Person;
use crate::validators::Validator;
use std::sync::Arc;
use crate::prompt::prompt;
use crate::utils::validator_loader::load_validators;
use crate::utils::create_defaults::default_string;
use std::{fs};
use std::path::Path;

fn read_validator_path_from_config(key: &str) -> String {
    let default_path = "resources/xml/account_validations/validator.xml";
    let config_path = "resources/config.properties";
    // declare a new path that is resolved as config_path or default path
    let path = if Path::new(config_path).exists() {
        config_path
    } else {
        default_path
    };

    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read config file '{}': {}", path, e);
            return default_path.to_string();
        }
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(idx) = line.find('=') {
            let tmp_key = line[..idx].trim();
            let val = line[idx + 1..].trim();
            if tmp_key == key {
                return val.to_string();
            }
        }
    }
    default_path.to_string()
}

fn read_config_bool(key: &str) -> bool {
    let config_path = "resources/config.properties";
    if !Path::new(config_path).exists() {
        return false;
    }
    let content = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(_) => return false,
    };
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(idx) = line.find('=') {
            let tmp_key = line[..idx].trim();
            let val = line[idx + 1..].trim();
            if tmp_key == key {
                return matches!(val.to_lowercase().as_str(), "true" | "1" | "yes" | "y" );
            }
        }
    }
    false
}

pub fn run_account_validation_demo() {
    let name = prompt("Owner name");
    let age: u32 = prompt("Owner age").parse().unwrap_or(0);
    let city = prompt("Owner city");
    let account_type_input = prompt("Account type (Normal/Premium)");
    let account_type: AccountType = account_type_input.parse().unwrap_or(AccountType::Other(account_type_input.clone()));
    let account_number_input = prompt("Account number (leave empty to auto-generate)");

    let account_number = default_string(account_number_input.trim(), || generate_account_number());

    let balance: f64 = prompt("Initial balance").parse().unwrap_or(0.0);

    let owner = Person { name, age, city };
    let account = Account::new(owner, account_type.clone(), balance, account_number.clone());

    let path = read_validator_path_from_config("validator.path");

    let validators_map: std::collections::HashMap<String, Vec<Arc<dyn Validator>>> = match load_validators(&path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to load validators: {}", e);
            return;
        }
    };
    let lookup_key = account.account_type.as_lowercase_key();
    if let Some(list) = validators_map.get(&lookup_key) {
        println!("Found {} validator(s) for '{}'", list.len(), account.account_type);
        let fail_fast = read_config_bool("fail.fast");
        if fail_fast {
            println!("Fail-fast enabled: will stop on first validation failure");
        }
        for v in list {
            let name = v.name();
            println!("Validating with: {}", name);
            match v.validate(&account) {
                Ok(_) => println!("Validator [ {} ] passed", name),
                Err(e) => {
                    println!("Validator [ {} ] failed: {}", name, e);
                    if fail_fast {
                        println!("Stopping further validation due to fail.fast=true");
                        break;
                    }
                }
            }
        }
    } else {
        println!("No validators configured for account type '{}'", account_type);
    }
}

fn generate_account_number() -> String {
    // Use milliseconds since UNIX_EPOCH to create a reasonably unique id.
    // Prepend with 'AC' to make it clear it's an account number.
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    // Keep last 10 digits to keep the id short but likely unique across calls.
    let s = now.to_string();
    let tail = if s.len() > 10 { &s[s.len() - 10..] } else { &s }
    ;
    format!("AC{}", tail)
}
