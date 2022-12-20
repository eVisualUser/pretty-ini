use crate::variable::Variable;
use crate::parser_config::ParserConfig;

#[derive(Default, Debug)]
pub struct Table {
    pub name: String,
    pub content: Vec<Variable>,
}

impl Table {
    pub fn get_variable_refmut(&mut self, name: &str) -> Option<&mut Variable> {
        for i in 0..self.content.len() {
            if self.content[i].name == name {
                return Some(&mut self.content[i]);
            }
        }
        None
    }

    pub fn get_variable_value(&self, name: String) -> String {
        for var in self.content.iter() {
            if var.name == name {
                return var.value.clone();
            }
        }
        return String::new();
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.content.push(variable);
    }

    pub fn as_vec_string(&self, parser_config: &ParserConfig) -> Vec<String> {
        let mut result = Vec::<String>::new();

        if self.name != crate::ini::TABLE_NAME_ROOT {
            result.push(format!("{}{}{}", parser_config.section_closure.open, self.name, parser_config.section_closure.close));
        }

        for var in self.content.iter() {
            if var.unknow_element.is_none() {
                result.push(format!("{} {} {}", var.name, parser_config.define_char, var.value));
            } else {
                result.push(var.unknow_element.clone().unwrap());
            }
        }

        result.push(String::new());

        result
    }
}
