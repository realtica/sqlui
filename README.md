# sqlui

![Crates.io](https://img.shields.io/crates/v/sqlui)
![Crates.io](https://img.shields.io/crates/d/sqlui)
![Crates.io](https://img.shields.io/crates/l/sqlui)

### Video Demo: https://youtu.be/cb1F_MkoCoE

### Description: 

The sqlui crate is blazing fast, lightweight, small binary and crossplatform, database client.

### Why choose sqlui?

- Very fast database viewer.
- Only uses ~20MB of RAM.
- Very low use of CPU.
- Crossplatform and compatible with Single-board computer like Raspberry.

### Installation

```
cargo install sqlui
```

### Configuration

Please create a config.toml into system config path:

**Path example:**

```
(Linux) /home/alice/.config/sqlui/config.toml

(Windows) C:\Users\Alice\AppData\Roaming\sqlui\config.toml

(Mac) /Users/Alice/Library/Application Support/sqlui/config.toml
```

**config.toml example**

```
[[endpoints]]
name = "employees"
connection_string = "mysql://root:college@localhost:3366/employees"

[[endpoints]]
name = "world"
connection_string = "mysql://root:local@localhost:3356/world"
```
