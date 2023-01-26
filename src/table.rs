use crate::parser_config::ParserConfig;
use crate::variable::Variable;

#[derive(Default, Debug, Clone)]
pub struct Table {
    pub name: String,
    pub content: Vec<Variable>,
}

impl Table {
    pub fn get_variable_ref_mut(&mut self, key: &str) -> Result<&mut Variable, String> {
        for i in 0..self.content.len() {
            if self.content[i].key == key {
                return Ok(&mut self.content[i]);
            }
        }
        Err(format!("Variable [{}] not found", key))
    }

    pub fn get_variable_ref(&self, key: &str) -> Result<&Variable, String> {
        for i in 0..self.content.len() {
            if self.content[i].key == key {
                return Ok(&self.content[i]);
            }
        }
        Err(format!("Variable [{}] not found", key))
    }

    pub fn get_variable(&self, key: &str) -> Result<Variable, String> {
        for i in 0..self.content.len() {
            if self.content[i].key == key {
                return Ok(self.content[i].clone());
            }
        }
        Err(format!("Variable [{}] not found", key))
    }

    pub fn get_variable_value(&self, key: String) -> String {
        for var in self.content.iter() {
            if var.key == key {
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
            result.push(format!(
                "{}{}{}",
                parser_config.section_closure.open, self.name, parser_config.section_closure.close
            ));
        }

        for var in self.content.iter() {
            if var.unknow_element.is_none() {
                result.push(format!(
                    "{} {} {}",
                    var.key, parser_config.define_char, var.value
                ));
            } else {
                result.push(var.unknow_element.clone().unwrap());
            }
        }

        result.push(String::new());

        result
    }
}
