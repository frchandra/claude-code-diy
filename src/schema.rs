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
    Chat(MessageFromHuman),
    Tool(MessageFromTool),
    Assistant(MessageFromAssistant),
}

#[derive(Serialize)]
pub struct MessageFromHuman {
    pub role: String,
    pub content: String,
}

impl MessageFromHuman {
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }
}

#[derive(Serialize)]
pub struct MessageFromTool {
    pub role: String,
    pub tool_call_id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct MessageFromAssistant {
    pub role: String,
    pub content: String,
    pub tool_calls: Vec<RequestForTool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestForTool {
    pub id: String,
    pub function: FunctionInvocation,
    #[serde(rename = "type")]
    pub tool_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionInvocation {
    pub name: String,
    pub arguments: String,
}

impl FunctionInvocation {
    pub fn get_argument(&self) -> Result<FunctionInvocationArgs, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FunctionInvocationArgs {
    pub file_path: Option<String>,    
    pub content: Option<String>,
    pub command : Option<String>,
}

#[derive(Serialize)]
pub struct ToolSpec {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionSpec,
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
                        file_path: Some(PropertySpec {
                            schema_type: "string".to_string(),
                            description: "The path to the file to read".to_string(),
                        }),
                        content: None,
                        command: None,
                    },
                    required: vec!["file_path".to_string()],
                },
            },
        }
    }

    pub fn write_file_tool() -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionSpec {
                name: "Write".to_string(),
                description: "Write content to a file".to_string(),
                parameters: ParametersSpec {
                    schema_type: "object".to_string(),
                    properties: PropertiesSpec {
                        file_path: Some(PropertySpec {
                            schema_type: "string".to_string(),
                            description: "The path to the file to write".to_string(),
                        }),
                        content: Some(ContentSpec {
                            content_type: "string".to_string(),
                            description: "The content to write to the file".to_string(),
                        }),
                        command: None,
                    },
                    required: vec!["file_path".to_string(), "content".to_string()],
                },
            },
        }
    }

    pub fn run_bash() -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionSpec {
                name: "Bash".to_string(),
                description: "Run a shell command and return its output".to_string(),
                parameters: ParametersSpec {
                    schema_type: "object".to_string(),
                    properties: PropertiesSpec {
                        file_path: None,
                        content: None,
                        command: Some(CommandSpec {
                            command_type: "string".to_string(),
                            description: "The shell command to execute".to_string(),
                        }),
                    },
                    required: vec!["command".to_string()],
                },
            },
        }

    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<PropertySpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandSpec>,

}

#[derive(Serialize)]
pub struct PropertySpec {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ContentSpec {
    #[serde(rename = "type")]
    pub content_type: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct CommandSpec {
    #[serde(rename = "type")]
    pub command_type: String,
    pub description: String,
}
