# BlueprintOS — Project Plan (Agentic Voice-First OS)

## Vision Shift
BlueprintOS is no longer a general-purpose OS. It's now an **agentic voice-first intelligent layer** on Linux, powered by Qwen3 Omni for multimodal understanding.

## Core Architecture Decisions

1. **Linux base** — Distribution-agnostic layer (Ubuntu/Debian as reference)
2. **Voice-first UX** — No traditional GUI; voice + visual output only
3. **Qwen3 Omni** — Central AI brain for all interactions
4. **Agent bus** — Message-passing architecture for task orchestration

## Phase-Based Roadmap

### Phase 1 — Speech-to-Speech Foundation (Weeks 1‑2)
| Task | Owner | Deliverable |
|------|-------|-------------|
| Wake word detection ("Hey Blueprint") | STS | Rust binary, <500ms latency |
| Audio capture/playback (cpal) | STS | Cross-platform mic/speaker I/O |
| Qwen3 Omni STS API client | STS | Rust client, streaming audio in/out |
| Chunked audio streaming pipeline | STS | <200ms latency from speech to response |
| Basic agent loop (audio → Qwen → audio) | Core | Working STS demo |

### Phase 2 — Qwen3 Omni Integration (Weeks 3‑4)
| Task | Owner | Deliverable |
|------|-------|-------------|
| Qwen3 Omni API client | Core | Rust crate for streaming inference |
| Multimodal input handling (voice + text) | Core | Unified input pipeline |
| Streaming response processing | Core | Real-time TTS + partial visual updates |
| Context memory across sessions | Core | SQLite persistence |

### Phase 3 — Visual Layer (Weeks 5‑6)
| Task | Owner | Deliverable |
|------|-------|-------------|
| Canvas-based UI renderer | UI | HTML5 Canvas, smooth animations |
| Agent thought visualization | UI | Streaming token display, confidence scores |
| Status indicators (system health) | UI | Charts, progress bars, animations |
| Error visualization | UI | Friendly voice + visual feedback |

### Phase 4 — Agent Ecosystem (Weeks 7‑10)
| Agent | Capabilities |
|-------|-------------|
| `agent-files` | File search, read, write, organize |
| `agent-system` | Process management, memory, CPU monitoring |
| `agent-network` | Connectivity checks, curl, ping |
| `agent-shell` | Execute commands, return structured output |
| `agent-calendar` | Event creation, query, reminders |

### Phase 5 — Distribution (Weeks 11‑12)
| Task | Owner | Deliverable |
|------|-------|-------------|
| Installer script (Linux overlay) | DevOps | `./install.sh` — single-command setup |
| Base image (Docker/VM) | DevOps | Pre-built BlueprintOS image |
| Documentation | Docs | Voice command reference, architecture guide |
| Community setup | All | Issues, PR templates, contributing guide |

## Repository Structure

```
blueprintos/
├── README.md
├── PROJECT_PLAN.md
├── LICENSE
├── .github/
│   └── workflows/
│       └── ci.yml
├── install.sh                 # Main installer
├── blueprint-core/            # Rust agent orchestrator
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── agent/
│       ├── bus/
│       └── cli/
├── blueprint-voice/           # Python voice pipeline
│   ├── blueprint_voice/
│   │   ├── stt.py
│   │   ├── tts.py
│   │   ├── wakeword.py
│   │   └── __init__.py
│   └── requirements.txt
├── blueprint-ui/              # Visual output layer
│   ├── index.html
│   ├── styles.css
│   └── app.js
├── blueprint-agents/          # Task-specific agents
│   ├── files/
│   ├── system/
│   ├── network/
│   └── shell/
└── docs/
    ├── voice-commands.md
    └── architecture.md
```

## Immediate Next Actions

1. **Create Rust core crate** (`blueprint-core`) with agent bus
2. **Create Python voice package** (`blueprint-voice`) with STT/TTS stubs
3. **Create HTML/JS UI** (`blueprint-ui`) with Canvas rendering
4. **Set up CI** — Rust lint + test, Python lint + test
5. **Write `install.sh`** — Detect Linux, install dependencies, clone if needed

## Technology Choices

| Layer | Tech | Rationale |
|-------|------|-----------|
| Core | Rust | Safe, fast, great for agent orchestration |
| Voice | Python | Rich STT/TTS ecosystem, easy prototyping |
| UI | HTML5 Canvas | Cross-platform, easy to embed, smooth animations |
| Runtime | Linux native | Runs on any distro or as Docker container |

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Qwen3 Omni API rate limits | Medium | Implement caching, graceful fallback to local STT |
| Voice latency >1s | High | Optimize wake word, stream partial results early |
| UI performance on low-end devices | Medium | Lazy rendering, fallback to text-only mode |

## Success Criteria

- [ ] Voice command → response in <2 seconds (average)
- [ ] Wake word detection accuracy >95%
- [ ] 5 functional task agents (files, system, network, shell, calendar)
- [ ] Visual UI renders at 60fps on reference hardware
- [ ] Installable on clean Ubuntu 22.04 in <10 minutes

---

**Ready to start?** I can scaffold all crates and services, set up the GitHub repo (already created), and push the initial commit.
