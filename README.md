# BlueprintOS

**BlueprintOS** — An agentic operating system with voice-first and visual UI, powered by **Qwen3 Omni**. Built as an intelligent layer on top of Linux.

## 🎯 Vision

BlueprintOS reimagines the OS experience:
- **No traditional GUI** — Interact through voice and visuals only
- **Agentic by default** — Every task is handled by AI agents
- **Qwen3 Omni powered** — Multimodal understanding (voice, text, vision)
- **Linux foundation** — Runs on any Linux distro or bare metal

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    BlueprintOS Layer                        │
├─────────────────────────────────────────────────────────────┤
│                    ┌─────────────────────┐                  │
│                    │   Speech-to-Speech  │                  │
│                    │   (Qwen3 Omni)      │                  │
│                    │   Real-time Audio   │                  │
│                    └──────────┬──────────┘                  │
│                               │                             │
│              ┌────────────────┼────────────────┐            │
│              ▼                ▼                ▼            │
│     ┌─────────────┐   ┌─────────────┐   ┌─────────────┐    │
│     │  Audio In   │   │ Agent Bus   │   │  Visual UI  │    │
│     │ (Mic Stream)│   │ (Message)   │   │ (Canvas)    │    │
│     └─────────────┘   └─────────────┘   └─────────────┘    │
│                                                          │
├──────────────────────────┬───────────────────────────────┤
│                          ▼                                │
│             ┌─────────────────────────────┐               │
│             │    Linux Base (Any Distro)  │               │
│             │   + Audio Drivers + GPU     │               │
│             └─────────────────────────────┘               │
└─────────────────────────────────────────────────────────────┘
```

## 🎤 Speech-to-Speech UX

- **Single streaming pipeline**: Audio in → Qwen3 Omni → Audio out
- **No STT/TTS separation**: One model handles both directions
- **Ultra-low latency**: Chunked audio streaming for real-time feel
- **Voice preservation**: Speaker tone and emotion maintained
- **Wake word**: "Hey Blueprint" triggers listening mode
- **Visual feedback**: Canvas shows agent thought process, results, animations

## 🧠 Qwen3 Omni - Speech-to-Speech

Qwen3 Omni natively supports real-time speech-to-speech:
- **Multimodal streaming**: Voice + text + images in/out
- **Single API call**: No separate transcription/synthesis
- **Native audio output**: Direct PCM/audio stream response
- **Context awareness**: Maintains conversation history

## 📦 Core Components

| Component | Language | Description |
|-----------|----------|-------------|
| `blueprint-core` | Rust | Agent orchestration, message bus |
| `blueprint-sts` | Rust/Python | Speech-to-Speech streaming with Qwen3 Omni |
| `blueprint-ui` | HTML/JS/Canvas | Visual output, animations, status display |
| `blueprint-cli` | Rust | Terminal fallback for developers |
| `blueprint-installer` | Shell | Install BlueprintOS layer on Linux |

## 🚀 Getting Started

### Prerequisites
- Linux (Ubuntu 22.04+ or similar)
- Python 3.10+
- Rust 1.75+
- Microphone & speakers

### Quick Install

```bash
# Clone the repo
git clone https://github.com/sndrkrshnn/blueprintos.git
cd blueprintos

# Run installer
./install.sh

# Start BlueprintOS
blueprintos start
```

### Interacting

```
User: "Hey Blueprint, what's my system status?"
BlueprintOS: "Your system is healthy. 8GB RAM used of 16GB..."
[Visual: Shows RAM usage chart]

User: "Find all PDF files modified this week"
BlueprintOS: "Searching..."
[Visual: Displays file results in a grid]
```

## 🎯 Roadmap

| Phase | Goal | Deliverables |
|-------|------|--------------|
| **1** | Speech-to-Speech foundation | Wake word, audio streaming, Qwen3 Omni STS |
| **2** | Real-time integration | Chunked audio streaming, low-latency pipeline |
| **3** | Visual layer | Canvas-based UI, animations, status display |
| **4** | Agent ecosystem | Task agents (files, system, network, etc.) |
| **5** | Distribution | Installer, base images, documentation |

## 🔧 Development

```bash
# Setup development environment
make setup

# Run core agent (Rust)
cd blueprint-core && cargo run -- --sts

# Run Speech-to-Speech service
cd blueprint-sts && cargo run -- --api-key YOUR_KEY

# Run visual UI (browser)
cd blueprint-ui && python -m http.server 8080
```

## 📄 License

MIT License — see LICENSE file.

## 🤝 Contributing

This is early-stage. Ideas, PRs, and feedback welcome!

---

**BlueprintOS** — An OS that listens, sees, and acts.
