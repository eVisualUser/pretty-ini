use std::str::FromStr;
use std::fmt::Debug;

#[derive(Default, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub uknow_element: Option<String>,
}

impl Variable {
    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.value.parse::<T>()
    }
}
