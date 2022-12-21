use std::str::FromStr;
use std::fmt::{Debug, Display};

#[derive(Default, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub unknow_element: Option<String>,
}

impl Variable {
    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.value.parse::<T>()
    }

    pub fn set<T: Display>(&mut self, value: T) {
        self.value = format!("{}", value);
    }
}
