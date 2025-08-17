
/**
 * Want to create couple of default methods that take a lambda as param for default implementation.
 * Otherwise, return the actual value.
 */

 // need a default implementation for string if it is not empty, return it otherwise, return the default one
pub fn default_string<F>(value: &str, f: F) -> String
where
    F: FnOnce() -> String,
{
    if !value.is_empty() {
        value.to_string()
    } else {
        f()
    }
}