# Claude Code DIY

Create your own Claude Code!

## Step 1

Communicate with the
llm: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request)<br>
Try this: <br>
`./run.sh --prompt "PING"`<br>

## Step 2

Advertise the
tools: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
Try this: <br>
`./run.sh --prompt "How many tools are available to you in this request? Number only."`

## Step 3

Try to execute the
tools: [reference](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request#request.body.tools)<br>
Try this: <br>
`./run.sh --prompt -p "Read `apple.py
` and return its contents. Respond with only file contents, no surrounding text/backticks."`

<br>_Nismara Chandra, April 2026_