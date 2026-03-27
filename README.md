# MiniMax CLI

A Rust CLI application for MiniMax AI APIs - text-to-speech, voice cloning, video generation, image generation, and music generation.

## Features

- **Text-to-Speech**: Convert text to audio with various voices
- **Voice Cloning**: Clone a voice from an audio file
- **Voice Design**: Create custom voices from descriptive prompts
- **Video Generation**: Generate videos from text/image prompts (async with task tracking)
- **Image Generation**: Generate images from text prompts
- **Music Generation**: Generate music from prompt and lyrics
- **Task Management**: Track long-running async tasks with SQLite storage

## Installation

### From Source

```bash
git clone https://github.com/MiniMax-AI/MiniMax-CLI.git
cd MiniMax-CLI
cargo build --release
./target/release/minimax --help
```

## Configuration

### Quick Setup

Run the interactive configuration wizard:

```bash
minimax config init
```

This will guide you through setting up your API key and selecting your region.

### Manual Configuration

```bash
# Set your API key (stored securely in system keyring)
minimax config set-api-key your-api-key-here

# Set API host
minimax config set-api-host https://api.minimax.io  # or https://api.minimaxi.com for China

# View current configuration
minimax config show
```

### Config File Location

Configuration is stored in:
- **Linux**: `~/.config/minimax-cli/config.toml`
- **macOS**: `~/Library/Application Support/minimax-cli/config.toml`
- **Windows**: `%APPDATA%/minimax-cli/config.toml`

Your API key is stored securely in your system's keyring/credential manager.

### API Keys

| Region | API Key | API Host |
|--------|---------|----------|
| Global | [MiniMax Global](https://www.minimax.io/platform/user-center/basic-information/interface-key) | `https://api.minimax.io` |
| China | [MiniMax China](https://platform.minimaxi.com/user-center/basic-information/interface-key) | `https://api.minimaxi.com` |

**Important**: API key and host must be from the same region.

## Usage

### Text to Audio

```bash
minimax text-to-audio --text "Hello, world!" --voice-id female-shaonv
```

### List Voices

```bash
minimax list-voices
minimax list-voices --voice-type system
```

### Voice Clone

```bash
minimax voice-clone --voice-id my-voice --file audio.mp3
minimax voice-clone --voice-id my-voice --file https://example.com/audio.mp3 --is-url
```

### Voice Design

```bash
minimax voice-design --prompt "Young female voice" --preview-text "Hello"
```

### Generate Video (Async)

```bash
minimax generate-video --prompt "A cat playing piano" --model MiniMax-Hailuo-02
```

This returns a task ID immediately. Use `query-task` to check status.

### Query Task Status

```bash
minimax query-task --task-id <task_id>
```

### Download Completed Task

```bash
minimax download-task --task-id <task_id> --output-dir ./downloads
```

### List All Tasks

```bash
minimax list-tasks
minimax list-tasks --status success --limit 10
```

### Text to Image

```bash
minimax text-to-image --prompt "A beautiful sunset" --aspect-ratio 16:9 --n 2
```

### Music Generation

```bash
minimax music-generation --prompt "Pop, happy" --lyrics "Hello world\nSecond line"
```

## Async Video/Image Generation

Long-running operations (video generation) immediately return a task ID and store it in the local SQLite database. You can:

1. **Check status**: `minimax query-task --task-id <id>`
2. **Download when ready**: `minimax download-task --task-id <id>`
3. **List all tasks**: `minimax list-tasks`

## Models

| Task | Default Model | Options |
|------|--------------|---------|
| Text-to-Audio | `speech-2.6-hd` | |
| Video | `MiniMax-Hailuo-2.3` | `T2V-01`, `MiniMax-Hailuo-02`, `I2V-01` |
| Image | `image-01` | |
| Music | `music-2.0` | |

## License

MIT License - see [LICENSE](LICENSE) for details.