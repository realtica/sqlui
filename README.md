## Installation

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
