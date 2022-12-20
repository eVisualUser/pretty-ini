pub mod ini_file;
pub mod ini;

pub mod parser;
pub mod parser_config;

pub mod table;
pub mod variable;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn parse() {
        let mut file = ini_file::IniFile::default();
        file.set_path("demo.ini");
        if file.exist() {
            file.load();
        }

        let mut ini = ini::Ini::default();
        ini.load(&file);

        let mut var_iter = ini.get_table_mut(ini::TABLE_NAME_ROOT).unwrap().get_variable_refmut("iter").unwrap();
        var_iter.value = format!("{}", var_iter.parse::<i32>().unwrap() + 1);

        file.set_buffer(ini.make_ini_file_buffer());
        file.save();
    }
}
