# ai-chat

A small Rust command-line program that has Claude and ChatGPT talk to each other.

It sends an opening prompt to Claude, passes Claude's reply to ChatGPT, passes ChatGPT's reply back to Claude, and so on for a fixed number of rounds. Each message is printed to the terminal as it arrives.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)
- An [Anthropic API key](https://console.anthropic.com/)
- An [OpenAI API key](https://platform.openai.com/api-keys)

## Setup

Create a `.env` file in the project root with both API keys:

```sh
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-...
```

The program loads this file at startup using the [dotenvy](https://crates.io/crates/dotenvy) crate and exits with an error if either key is missing. Variables already exported in your shell take precedence over the values in `.env`.

`.env` is listed in `.gitignore`, so your keys stay out of the repository.

## Usage

```sh
cargo run
```

Example output:

```
OPENER: Raquel is struggling finding a job. ...

CLAUDE: ...

CHATGPT: ...
```

## Configuration

Settings are constants at the top of [src/main.rs](src/main.rs):

| Constant       | Default             | Description                                         |
| -------------- | ------------------- | --------------------------------------------------- |
| `CLAUDE_MODEL` | `claude-sonnet-5-5` | Anthropic model to use                              |
| `OPENAI_MODEL` | `gpt-5`             | OpenAI model to use                                 |
| `TURNS`        | `5`                 | Number of rounds (each round is one reply per model) |
| `OPENER`       | job-search question | The prompt that starts the conversation             |

Each model also gets a short system prompt telling it that it is talking to another AI and should keep its replies short. You can change these in `ask_claude` and `ask_openai`.

## How it works

The program keeps a separate message history for each model. Each model sees its own replies with the `assistant` role and the other model's replies with the `user` role. This way each API receives a normal user/assistant conversation.

- Claude is called through the [Messages API](https://docs.claude.com/en/api/messages) (`POST /v1/messages`).
- ChatGPT is called through the [Chat Completions API](https://platform.openai.com/docs/api-reference/chat) (`POST /v1/chat/completions`).

Both APIs charge per token. With the default settings, one run makes 10 API calls, and the history sent with each call grows as the conversation goes on.
