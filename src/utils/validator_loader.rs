use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use crate::validators::get_validator_by_name;
use crate::validators::Validator;
use serde_json;


pub fn load_validators(path: &str) -> Result<HashMap<String, Vec<Arc<dyn Validator>>>, Box<dyn std::error::Error>> {
    // Dispatch based on file extension. If it's .json use the JSON loader, otherwise fall back to XML.
    if let Some(ext) = Path::new(path).extension().and_then(|s| s.to_str()) {
        if ext.eq_ignore_ascii_case("json") {
            return load_validators_from_json(path);
        }
    }

    load_validators_from_xml(path)
}

fn load_validators_from_json(path: &str) -> Result<HashMap<String, Vec<Arc<dyn Validator>>>, Box<dyn std::error::Error>> {
    // Read file
    let content = std::fs::read_to_string(path)?;

    // Expect a mapping of account type -> array of validator names, e.g. { "Normal": ["..."] }
    let map: HashMap<String, Vec<String>> = serde_json::from_str(&content)?;

    let mut validators: HashMap<String, Vec<Arc<dyn Validator>>> = HashMap::new();
    let mut registry: HashMap<String, Arc<dyn Validator>> = HashMap::new();

    for (account_type, names) in map {
        let key = account_type.trim().to_lowercase();
        validators.entry(key.clone()).or_insert_with(Vec::new);
        for class_name in names {
            let class_name = class_name.trim().to_string();
            let validator_arc = if let Some(existing) = registry.get(&class_name) {
                existing.clone()
            } else {
                match get_validator_by_name(&class_name) {
                    Some(b) => {
                        let arc: Arc<dyn Validator> = Arc::from(b);
                        registry.insert(class_name.clone(), arc.clone());
                        arc
                    }
                    None => {
                        eprintln!("No validator implementation wired for '{}'", class_name);
                        continue;
                    }
                }
            };
            validators.entry(key.clone()).or_default().push(validator_arc);
        }
    }

    Ok(validators)
}

fn load_validators_from_xml(path: &str) -> Result<HashMap<String, Vec<Arc<dyn Validator>>>, Box<dyn std::error::Error>> {
    // Read file as string for debug visibility
    let content = std::fs::read_to_string(path)?;

    // println!("[validator_loader] reading file: {}", path);
    // println!("[validator_loader] file content:\n{}", content);
    
    let mut reader = Reader::from_reader(content.as_bytes());
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut validators: HashMap<String, Vec<Arc<dyn Validator>>> = HashMap::new();
    let mut current_account: Option<String> = None;
    // registry to avoid creating multiple instances for the same validator type
    let mut registry: HashMap<String, Arc<dyn Validator>> = HashMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"Account" {
                    if let Some(val) = e.attributes().with_checks(false).find(|a| match a {
                        Ok(at) => at.key.as_ref() == b"type",
                        _ => false,
                    }) {
                        if let Ok(a) = val {
                            if let Ok(v) = a.unescape_value() {
                                let key = v.to_string().trim().to_lowercase();
                                current_account = Some(key);
                                validators.entry(current_account.clone().unwrap()).or_insert(vec![]);
                            }
                        }
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"class" {
                    if let Some(acc) = current_account.clone() {
                        if let Some(val) = e.attributes().with_checks(false).find(|a| match a {
                            Ok(at) => at.key.as_ref() == b"name",
                            _ => false,
                        }) {
                            if let Ok(a) = val {
                                if let Ok(v) = a.unescape_value() {
                                    let class_name = v.to_string().trim().to_string();
                                    // reuse an Arc'd instance if already created
                                    let validator_arc = if let Some(existing) = registry.get(&class_name) {
                                        existing.clone()
                                    } else {
                                        match get_validator_by_name(&class_name) {
                                            Some(b) => {
                                                let arc: Arc<dyn Validator> = Arc::from(b);
                                                registry.insert(class_name.clone(), arc.clone());
                                                arc
                                            }
                                            None => {
                                                eprintln!("No validator implementation wired for '{}'", class_name);
                                                continue;
                                            }
                                        }
                                    };
                                    validators.entry(acc).or_default().push(validator_arc);
                                }
                            }
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"Account" {
                    current_account = None;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(validators)
}
