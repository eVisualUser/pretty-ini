use crate::parser_config::ParserConfig;
use crate::variable::Variable;

pub enum ElementType {
    Unknow,
    Table,
    Variable,
}

#[derive(Default, Debug)]
pub struct Parser {
    pub config: ParserConfig,
}

impl Parser {
    pub fn new(config: ParserConfig) -> Self {
        Self {
            config,
        }
    }

    pub fn get_element_type(&self, input: &str) -> ElementType {
        if input.starts_with('[') {
            return ElementType::Table;
        } else if input.contains('=') {
            return ElementType::Variable;
        }

        return ElementType::Unknow;
    }

    pub fn parse_variable(&self, variable: &str) -> Variable {
        let mut name = String::new();
        let mut value = String::new();

        let mut is_name = true;
        for i in variable.chars() {
            if i == self.config.define_char {
                is_name = false;
            }
            else if is_name {
                name.push(i);
            } else {
                value.push(i);
            }
        }

        name = name.trim().to_string();
        value = value.trim().to_string();

        Variable { name, value, unknow_element: None }
    }

    pub fn parse_table_title(&self, title: &str) -> String {
        let mut result = String::new();
        for i in title.chars() {
            if !(i == self.config.section_closure.open || i == self.config.section_closure.close) {
                result.push(i);
            }
        }
        result.trim().to_string()
    }
}
