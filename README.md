# rust_learnings

A small Rust project for learning and experimenting with CSV reading and basic data modeling.

## Table of Contents

- Quick Start
  - Build
  - Run
- Project Structure
  - main.rs
  - person.rs
  - read_from_csv.rs
- Data Files
- Usage Examples
- Contributing
- License

## Quick Start

### Build

Build the project with Cargo:

```powershell
cargo build
```

### Run

Run the binary with Cargo:

```powershell
cargo run
```

## Project Structure

- `Cargo.toml` - project manifest and dependencies.
- `src/` - source files:
  - `main.rs` - entry point.
  - `person.rs` - `Person` struct and related logic.
  - `read_from_csv.rs` - CSV reading helpers.
  - `validation_impl.rs` - interactive account validation demo that uses a plugin-style validator registry.
  - `utils/validator_loader.rs` - loads validator configurations from XML or JSON and wires them to concrete validator implementations.

## Data Files

- `people.csv` and `people_actual.csv` - sample CSV data used by the project.

## Usage Examples

See `src/read_from_csv.rs` and `src/main.rs` for example code that loads CSVs and constructs `Person` values.

## Plugin-based validation framework

This project includes a small plugin-style validation framework for `Account` objects. Validators are simple objects that implement the `Validator` trait and are registered by name.

Configuration
- `resources/config.properties` controls runtime behavior. Relevant keys:
  - `validator.path` — path to validator configuration file (JSON or XML). Example: `resources/json/account_validations/validator.json`.
  - `fail.fast` — when `true`, stop validation on the first failing validator.

Validator files
- JSON: a mapping of account type -> array of validator type names. Example:

```json
{
  "Normal": ["NormalBalanceValidator", "NormalDiscountValidator"],
  "Premium": ["PremiumBalanceValidator", "PremiumDiscountValidator"]
}
```

- XML: compatible structure under `resources/xml/account_validations/validator.xml` (legacy support).

How the interactive demo prompts (CLI)
- On startup the demo presents a menu:
  1) Read from csv — run the CSV-reading utilities to show sample data.
  2) Account Validation Demo — interactive flow to create an `Account` and validate it.

When you choose `Account Validation Demo` the program prompts for:
- `Owner name` — string used for the `Account.owner` field.
- `Owner age` — parsed as integer.
- `Owner city` — string.
- `Account type (Normal/Premium)` — parsed into the `AccountType` enum; unknown values become `Other("...")`.
- `Account number (leave empty to auto-generate)` — you may provide a number or leave blank to auto-generate a short unique id (prefixed with `AC`).
- `Initial balance` — parsed as f64.

What each task does
- Read from csv: reads the sample CSV files in `resources/data/` and prints/returns records (see `src/read_from_csv.rs`).
- Account Validation Demo: constructs an `Account` from prompts, reads the validator configuration file (path from `config.properties`), loads validator instances by name (the project wires names to concrete validator types in `src/validators/mod.rs`), and runs each validator.
  - If `fail.fast=true`, validation stops at the first failing validator and the program reports which validator failed and why.
  - Validation output lists which validators ran and whether they passed or failed.

Extending validators
- Add a new validator by implementing the `Validator` trait under `src/validators/`, and wire its name in `src/validators/mod.rs` in the `get_validator_by_name` helper.
- Add the validator name to the JSON/XML configuration under the appropriate account type.

Troubleshooting
- If no validators are found for an account type the demo prints `No validators configured for account type 'X'`.
- Unknown validator names are logged to stderr when the loader cannot resolve a name to a concrete implementation.


## License

See `Cargo.toml` for license information.
