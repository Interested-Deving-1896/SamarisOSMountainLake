#[derive(Debug, Clone)]
pub struct ExplainDecision {
    pub parameter: String,
    pub value: String,
    pub reason: String,
    pub was_clamped: bool,
    pub original_value: Option<String>,
}

impl ExplainDecision {
    pub fn new(parameter: impl Into<String>, value: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            parameter: parameter.into(),
            value: value.into(),
            reason: reason.into(),
            was_clamped: false,
            original_value: None,
        }
    }

    pub fn clamped(
        parameter: impl Into<String>,
        value: impl Into<String>,
        reason: impl Into<String>,
        original_value: impl Into<String>,
    ) -> Self {
        Self {
            parameter: parameter.into(),
            value: value.into(),
            reason: reason.into(),
            was_clamped: true,
            original_value: Some(original_value.into()),
        }
    }
}
