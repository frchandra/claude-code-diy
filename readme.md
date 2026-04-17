# Claude Code DIY

Create your own Claude Code!

## Step 1

Communicate with the
llm: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request)<br>
- Try this: <br>
`./run.sh --prompt "PING"`<br>

## Step 2

Advertise the
tools: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
- Try this: <br>
`./run.sh --prompt "How many tools are available to you in this request? Number only."`

## Step 3

Try to execute the
tools: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
- Try this: <br>
`./run.sh --prompt -p "Read apple.py
and return its contents. Respond with only file contents, no surrounding text/backticks."` <br>
- Prerequisite: <br>
Create a file named `apple.py` in the same directory as `run.sh` with some content in it.

## Step 4 implement the Agent loop

- Try this: <br>
`./run.sh -p "Use README.md to determine the chemical expiry period in months. Number only.` <br>
- Prerequisite: <br>
Create a file named `README.md` in the same directory as `run.sh` with some content in it that includes reference to another file that contain the actual resolution to the chemical expiry period in months. This would require a "loops" that chains multiple tools together to arrive at the final answer.

<br>_Nismara Chandra, April 2026_