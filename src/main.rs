use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
            "model": "anthropic/claude-haiku-4.5",
            // "model": "minimax/minimax-m2.5:free",
            "tools": [
                {
                  "type": "function",
                  "function": {
                    "name": "Read",
                    "description": "Read and return the contents of a file",
                    "parameters": {
                      "type": "object",
                      "properties": {
                        "file_path": {
                          "type": "string",
                          "description": "The path to the file to read"
                        }
                      },
                      "required": ["file_path"]
                    }
                  }
                }
            ]
        }))
        .await?;

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");
    // Check if tool_calls is not null, if so, print the tool calls
    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        // Extract the function_name and  file_path
        // let function_name = &tool_calls[0]["function"]["name"];
        let function_arguments = &tool_calls[0]["function"]["arguments"];
        let file_path = function_arguments
            .as_str()
            .and_then(|args| serde_json::from_str::<Value>(args).ok())
            .and_then(|v| v["file_path"].as_str().map(|s| s.to_string()));
        if let Some(path) = file_path {
            match tokio::fs::read_to_string(path).await {
                Ok(file_content) => println!("{}", file_content),
                Err(_err) => println!("failed to read file"),
            }
        } else {
            println!("`file_path` not found in function arguments");
        }
    }

    if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}
