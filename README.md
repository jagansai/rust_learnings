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

## Data Files

- `people.csv` and `people_actual.csv` - sample CSV data used by the project.

## Usage Examples

See `src/read_from_csv.rs` and `src/main.rs` for example code that loads CSVs and constructs `Person` values.

## License

See `Cargo.toml` for license information.
