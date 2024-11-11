The Rust library for Zengin Code.

# Installation

```
cargo add zengin
```

# Usage

```rust
use zengin::Zengin;

let zengin = Zengin::new().unwrap();

if let Some(bank) = zengin.get_bank("0001") {
    println!("Found bank: {}", bank.name);
}

let banks = zengin.find_banks_by_name(".*みずほ.*").unwrap();
for bank in banks {
    println!("Found bank: {}", bank.name);
}
```