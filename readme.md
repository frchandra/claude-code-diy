# Claude Code DIY

Create your own Claude Code!

## Step 1 Communicate with the LLM

Communicate with the
LLM: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request)<br>

- Try this: <br>
  `./run.sh --prompt "PING"`<br>

## Step 2 Advertise the tools

Advertise the
tools: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
Advertise the tools that are available to the agent in the request. This would allow the agent to know what tools it can
use to perform tasks. For example, if the agent has access to a tool that allows it to read files, it can use that tool
to read files and extract information from them. <br>

- Try this: <br>
  `./run.sh --prompt "How many tools are available to you in this request? Number only."`

## Step 3 Try to execute the tools

Try to execute the
tools: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
Execute the tools that are available to the agent in the request. This would allow the agent to perform tasks that
require the use of tools. For example, if the agent has access to a tool that allows it to read files, it can use that
tool to read files and extract information from them. <br>

- Try this: <br>
  `./run.sh --prompt -p "Read apple.py
  and return its contents. Respond with only file contents, no surrounding text/backticks."` <br>
- Prerequisite: <br>
  Create a file named `apple.py` in the same directory as `run.sh` with some content in it.

## Step 4 Implement the Agent loop

Implement the Agent loop<br>
The agent loop allows the agent to make multiple calls to the LLM and tools in a single request. This would enable the
agent to perform more complex tasks that require multiple steps. For example, to determine the chemical expiry period in
months, the agent can first read the `README.md` file to find out which file contains the chemical expiry period
information, and then read that file to get the actual chemical expiry period in months. <br>

- Try this: <br>
  `./run.sh -p "Use README.md to determine the chemical expiry period in months. Number only.` <br>
- Prerequisite: <br>
  Create a file named `README.md` in the same directory as `run.sh` with some content in it that includes reference to
  another file that contain the actual resolution to the chemical expiry period in months. This would require a "loops"
  that chains multiple tools together to arrive at the final answer.

## Step 5 Implement the write function

Implement the write
function: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
Now the agent can read files, but it cannot write files. Implement the write function to allow the agent to write files.
This would enable the agent to create new files or modify existing files based on the instructions it receives. <br>

- Try this: <br>
- `./run.sh -p 'Use README.md to determine the chemical expiry period in months. Respond with only a number.'` <br>
- `/run.sh -p 'What is the content of pineapple.py? Respond with only file contents, no surrounding text/backticks.`
- `./run.sh -p 'How many tools are available to you in this request? Respond with only a number.`
- `./run.sh -p 'What is 10*7? Respond with only a number.`

<br>_Nismara Chandra, April 2026_