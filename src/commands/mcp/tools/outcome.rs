//! What a tool returns before it becomes a `tools/call` result.

use serde_json::{json, Value};

/// A tool result: structured data, an error flag and optional extra content items.
pub struct Outcome {
    pub value: Value,
    pub is_error: bool,
    pub extra: Vec<Value>,
}

impl Outcome {
    /// A successful result.
    pub fn ok(value: Value) -> Self {
        Self {
            value,
            is_error: false,
            extra: Vec::new(),
        }
    }

    /// A failure carrying a message.
    pub fn fail(message: impl Into<String>) -> Self {
        Self::fail_with(json!({"error": message.into()}))
    }

    /// A failure carrying structured data, such as the violations that blocked a step.
    pub fn fail_with(value: Value) -> Self {
        Self {
            value,
            is_error: true,
            extra: Vec::new(),
        }
    }

    /// Adds a content item after the JSON text.
    pub fn with_extra(mut self, item: Value) -> Self {
        self.extra.push(item);
        self
    }

    /// The MCP `tools/call` result object.
    pub fn into_result(self) -> Value {
        let text = serde_json::to_string_pretty(&self.value).unwrap_or_default();
        let mut content = vec![json!({"type": "text", "text": text})];
        content.extend(self.extra);
        json!({"content": content, "structuredContent": self.value, "isError": self.is_error})
    }
}
