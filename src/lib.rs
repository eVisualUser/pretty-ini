pub mod ini;
pub mod ini_file;

pub mod parser;
pub mod parser_config;

pub mod table;
pub mod variable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mut file = ini_file::IniFile::default();
        file.set_path("demo.ini");

        let mut ini = ini::Ini::default();
        ini.load(&mut file).unwrap();

        let var_iter = ini.get_refmut(ini::TABLE_NAME_ROOT, "iter").unwrap();
        var_iter.set(var_iter.parse::<i32>().unwrap() + 1);

        file.save(&mut ini);
    }
}
