use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequest {
  pub messages: Vec<Message>,
  pub model: String,
  pub tools: Vec<ToolSpec>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Message {
  Chat(ChatMessage),
  Tool(ToolMessage),
  Assistant(AssistantMessage),
}

#[derive(Serialize)]
pub struct ChatMessage {
  pub role: String,
  pub content: String,
}

#[derive(Serialize)]
pub struct ToolMessage {
  pub role: String,
  pub tool_call_id: String,
  pub content: String,
}

#[derive(Serialize)]
pub struct AssistantMessage {
  pub role: String,
  pub content: String,
  pub tool_calls: Vec<ToolCall>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
  pub id: String,
  pub function: FunctionCall,
  #[serde(rename = "type")]
  pub tool_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCall {
  pub name: String,
  pub arguments: String,
}

#[derive(Serialize)]
pub struct ToolSpec {
  #[serde(rename = "type")]
  pub tool_type: String,
  pub function: FunctionSpec,
}

#[derive(Serialize)]
pub struct FunctionSpec {
  pub name: String,
  pub description: String,
  pub parameters: ParametersSpec,
}

#[derive(Serialize)]
pub struct ParametersSpec {
  #[serde(rename = "type")]
  pub schema_type: String,
  pub properties: PropertiesSpec,
  pub required: Vec<String>,
}

#[derive(Serialize)]
pub struct PropertiesSpec {
  pub file_path: PropertySpec,
}

#[derive(Serialize)]
pub struct PropertySpec {
  #[serde(rename = "type")]
  pub schema_type: String,
  pub description: String,
}

impl ChatMessage {
  pub fn user(content: String) -> Self {
    Self {
      role: "user".to_string(),
      content,
    }
  }
}

impl ToolSpec {
  pub fn read_file_tool() -> Self {
    Self {
      tool_type: "function".to_string(),
      function: FunctionSpec {
        name: "Read".to_string(),
        description: "Read and return the contents of a file".to_string(),
        parameters: ParametersSpec {
          schema_type: "object".to_string(),
          properties: PropertiesSpec {
            file_path: PropertySpec {
              schema_type: "string".to_string(),
              description: "The path to the file to read".to_string(),
            },
          },
          required: vec!["file_path".to_string()],
        },
      },
    }
  }
}
