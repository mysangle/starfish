
#[derive(Clone, Debug, PartialEq)]
pub struct TextData {
    /// Actual text
    pub value: String,
}

impl Default for TextData {
    fn default() -> Self {
        Self::new()
    }
}

impl TextData {
    #[must_use]
    pub fn new() -> Self {
        Self { value: String::new() }
    }

    pub fn with_value(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}
