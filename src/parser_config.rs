#[derive(Default, Debug, Clone)]
pub struct Closure {
    pub open: char,
    pub close: char,
}

#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub section_closure: Closure,
    pub define_char: char,
}

impl Default for ParserConfig {
    fn default() -> Self {

        let mut section_closure = Closure::default();
        section_closure.open = '[';
        section_closure.close = ']';

        Self {
            section_closure,
            define_char: '=',
        }
    }
}

impl ParserConfig {
    pub fn get_section_open_char(&self) -> char {
        self.section_closure.open
    }

    pub fn get_section_close_char(&self) -> char {
        self.section_closure.close
    }

    pub fn get_define_char(&self) -> char {
        self.define_char
    }
}
