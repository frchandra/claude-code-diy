mod schema;
use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use schema::{
    ChatRequest, Message, MessageFromAssistant, MessageFromHuman, MessageFromTool, RequestForTool,
    ToolSpec,
};
use serde_json::Value;
use std::env;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

const OPENROUTER_BASE_URL_ENV: &str = "OPENROUTER_BASE_URL";
const OPENROUTER_API_KEY_ENV: &str = "OPENROUTER_API_KEY";
const DEFAULT_OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

fn build_config() -> Result<OpenAIConfig, Box<dyn std::error::Error>> {
    let base_url = env::var(OPENROUTER_BASE_URL_ENV)
        .unwrap_or_else(|_| DEFAULT_OPENROUTER_BASE_URL.to_string());

    let api_key = env::var(OPENROUTER_API_KEY_ENV).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "OPENROUTER_API_KEY is not set",
        )
    })?;

    Ok(OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key))
}

fn build_client(config: OpenAIConfig) -> Client<OpenAIConfig> {
    Client::with_config(config)
}

async fn execute_tool(request_for_tool: &RequestForTool) -> Result<Option<String>, std::io::Error> {
    let properties_spec = request_for_tool.function.get_argument().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid tool arguments JSON: {e}"),
        )
    })?;

    match request_for_tool.function.name.as_str() {
        "Read" => {
            let file_content = tokio::fs::read_to_string(&properties_spec.file_path).await?;
            Ok(Some(file_content))
        }
        "Write" => {
            let content = properties_spec.content.ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "missing `content` in Write tool arguments",
                )
            })?;

            tokio::fs::write(&properties_spec.file_path, content.clone()).await?;
            Ok(Some(content.to_string()))
        }
        _ => Ok(None),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = build_config()?;
    let client = build_client(config);

    let messages = vec![Message::Chat(MessageFromHuman::user(args.prompt))];
    let model = "anthropic/claude-haiku-4.5".to_string();
    // let model = "nvidia/nemotron-3-super-120b-a12b:free".to_string();
    let tools = vec![ToolSpec::read_file_tool(), ToolSpec::write_file_tool()];

    let mut request = ChatRequest {
        messages,
        model,
        tools,
    };

    loop {
        // println!("{}", serde_json::to_string_pretty(&request)?); //debug

        let response: Value = client
            .chat()
            .create_byot(serde_json::to_value(&request)?)
            .await?;

        // println!("{}", serde_json::to_string_pretty(&response)?); //debug

        //transform tool_calls from Value to Vec<ToolCall>
        let tool_calls = response["choices"][0]["message"]["tool_calls"]
            .as_array()
            .map(|calls| {
                calls
                    .iter()
                    .filter_map(|call| serde_json::from_value::<RequestForTool>(call.clone()).ok())
                    .collect::<Vec<RequestForTool>>()
            })
            .unwrap_or_default();

        if tool_calls.is_empty() {
            println!(
                "{}",
                response["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("")
            );
            break;
        }

        let assistant_reply = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default();
        request
            .messages
            .push(Message::Assistant(MessageFromAssistant {
                role: "assistant".to_string(),
                content: assistant_reply.to_string(),
                tool_calls: tool_calls.clone(),
            }));

        for tool_call in tool_calls {
            match execute_tool(&tool_call).await {
                Ok(Some(file_content)) => {
                    let tool_message = MessageFromTool {
                        role: "tool".to_string(),
                        tool_call_id: tool_call.id.clone(),
                        content: file_content,
                    };
                    request.messages.push(Message::Tool(tool_message));
                }
                Ok(None) => eprintln!("file_path not found in function arguments"),
                Err(_) => eprintln!("failed to read file"),
            }
        }
        // println!("{}", serde_json::to_string_pretty(&request)?); //debug
    }

    Ok(())
}
