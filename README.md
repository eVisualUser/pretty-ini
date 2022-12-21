# Pretty INI

Light library to read/write ini files.

## Format

### Header / Table / Section
[name]

### Variable

myVar = value

## Example

```rust
use pretty_ini::{ini_file, ini};

// File buffer
let mut file = ini_file::IniFile::default();
file.set_path("demo.ini");

// Load
let mut ini = ini::Ini::default();
ini.load(&mut file).unwrap();

// Access a var and add 1
let mut var_iter = ini.get_refmut(ini::TABLE_NAME_ROOT, "iter").unwrap();
var_iter.set(var_iter.parse::<i32>().unwrap() + 1);

// Save the file
file.save(&mut ini);
```

## Pre/Post Process

In the IniFile you can add some process using a ProcessAction.

### Pre Process

Called before assign the file content to the buffer.

```rust
let action = Some(Box::new(|buffer| {
    // Do nothing
    return buffer;
}));

ini_file.add_pre_process(action);
```

### Post Process

Called before saving the file.

```rust
let action = Some(Box::new(|buffer| {
    // Do nothing
    return buffer;
}));

ini_file.add_post_process(action);
```

## ⚠️ Warnings
- The output when saving will be reformated.
- Implicit "root" table.
