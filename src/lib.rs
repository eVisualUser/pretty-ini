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

        let var_iter = ini.get_ref_mut(ini::TABLE_NAME_ROOT, "iter").unwrap();
        var_iter.set(var_iter.parse::<i32>().unwrap() + 1);

        println!("All keys contained in: \"Next\"");
        for key in ini
            .get_all_keys_in_table("next")
            .expect("No key found in Next")
        {
            println!("- {} = {:?}", key, ini.get("next", &key).unwrap());
        }

        file.save(&mut ini);
    }
}
