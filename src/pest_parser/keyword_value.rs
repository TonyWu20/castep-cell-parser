use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct KeywordValue {
    order: usize,
    name: String,
    value: String,
}

impl Display for KeywordValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:>4}", self.name(), self.value())
    }
}

impl KeywordValue {
    pub fn new<V: Display>(order: usize, name: String, value: V) -> Self {
        Self {
            order,
            name,
            value: value.to_string(),
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleKeyword {
    order: usize,
    name: String,
}

impl SingleKeyword {
    pub fn new(order: usize, name: String) -> Self {
        Self { order, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn order(&self) -> usize {
        self.order
    }
}

impl Display for SingleKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
