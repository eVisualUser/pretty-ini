# Pretty INI

Light library to read/write ini files.

## Format

```ini
[table_name]
key = value
```

## Example

```rust
use pretty_ini::{ini, ini_file};

fn main() {
    let mut file = ini_file::IniFile::default();
    file.set_path("demo.ini");

    let mut ini = ini::Ini::default();
    ini.load(&mut file).unwrap();

    let var_iter = ini.get_ref_mut(ini::TABLE_NAME_ROOT, "iter").unwrap();
    var_iter.set(var_iter.parse::<i32>().unwrap() + 1);

    println!("All keys contained in: \"Next\"");
    for key in ini
        .get_all_keys_in_table("next")
        .expect("No key found in Next")
    {
        println!("- {}", key);
    }

    file.save(&mut ini);
}

```

---
<details>
<summary>Pre/Post Process</summary>

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
</details>
---
## ⚠️ Warnings
- The output when saving will be reformated.
- Implicit "root" table.
