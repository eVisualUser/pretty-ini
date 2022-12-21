# Pretty INI
Light library to read/write ini files.

## Format
- A variable and an header or a comment can't be on the same line
- A line that is not a variable or a header is an unknow element

## Example

<code lang="rust">
use pretty_ini::{ini_file, ini};

let mut file = ini_file::IniFile::default();
file.set_path("demo.ini");

let mut ini = ini::Ini::default();
ini.load(&mut file).unwrap();

let mut var_iter = ini.get_refmut(ini::TABLE_NAME_ROOT, "iter").unwrap();
var_iter.set(var_iter.parse::<i32>().unwrap() + 1);

file.save(&mut ini);
</code>
