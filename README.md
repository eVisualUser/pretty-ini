# Pretty INI
Light library to read/write ini files.

## Format
- A variable and an header or a comment can't be on the same line
- A line that is not a variable or a header is an unknow element

## Example

```
use pretty_ini::ini_file::IniFile;

let mut file = ini_file::IniFile::default();
file.set_path("file_name.ini");

if file.exist() {
    // Can Load
    file.load();
}

// Convert to an object easy to use
let mut ini = ini::Ini::default();
ini.load(&file);

// Access a variable in the root table
// the root table is a table made of element without any header
let mut var_iter = ini.get_table_mut(ini::TABLE_NAME_ROOT).unwrap().get_variable_refmut("iter").unwrap();
// Add 1 to the variable value
// var_iter.parse::<TYPE>() is a short way to parse the String
var_iter.value = format!("{}", var_iter.parse::<i32>().unwrap() + 1);

// Give the new file content to the IniFile
ini.set_buffer(ini.make_ini_file_buffer());
// Write the buffer to the file
ini.save();
```
