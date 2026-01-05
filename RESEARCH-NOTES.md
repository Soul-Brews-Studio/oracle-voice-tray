# Voice Tray v2 - Research Notes

*Generated: 2026-01-04 by 10 parallel research agents*

## Executive Summary

Voice Tray v2 is a **Tauri 2.0 macOS menu bar app** that provides centralized text-to-speech for Claude Code. It was **spun off from Nat-s-Agents on 2026-01-04** (1.7GB → 11MB) after graduating from the incubation system.

---

## Timeline

| Date | Event |
|------|-------|
| Jan 2, 2026 | v1 complete (HTTP only) in `ψ/incubate/voice-tray/` |
| Jan 2, 2026 | v2 started (MQTT backend) |
| Jan 3, 2026 | Dynamic lips icon + overnight bug fixes |
| Jan 4, 2026 | **Spun off to own repo** (`laris-co/voice-tray-v2`) |

---

## Architecture

```
Claude Code Hooks
       ↓
voice-tray-notify.sh (HTTP) or voice-tray-mqtt-notify.sh (MQTT)
       ↓
Voice Tray v2 (Tauri app, port 37779)
       ↓
macOS `say -v [voice] -r [rate]`
       ↓
Timeline UI + Audio Output
```

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework | Tauri 2.0 (Rust + WebView) |
| HTTP Server | Axum 0.7 |
| MQTT Client | rumqttc 0.24 |
| Async Runtime | Tokio |
| Frontend | Vanilla HTML/CSS/JS |
| Voice | macOS `say` command |
| Icons | PNG32 via ImageMagick |

---

## Features

- **Dual Input**: HTTP POST `/speak` + MQTT `voice/speak` topic
- **Voice Queue**: Sequential processing, no overlap
- **Timeline UI**: 320x400 popup from menu bar click
- **Dynamic Tray Icon**: Lips change (idle/speaking states)
- **Per-Agent Voices**: Configured in `scripts/agent-voices.toml`

### Agent Voice Config
| Agent | Voice | Rate |
|-------|-------|------|
| Main | Samantha | 190 wpm |
| Agent 1 | Daniel (British) | 220 wpm |
| Agent 2 | Karen (Australian) | 220 wpm |
| Agent 3 | Rishi (Indian) | 220 wpm |

---

## API

**HTTP (port 37779)**
```bash
# Queue voice
curl -X POST http://127.0.0.1:37779/speak \
  -H "Content-Type: application/json" \
  -d '{"text":"Hello","voice":"Samantha","rate":190}'

# Get timeline
curl http://127.0.0.1:37779/timeline

# Get status
curl http://127.0.0.1:37779/status
```

**MQTT (broker 127.0.0.1:1883)**
```bash
mosquitto_pub -t voice/speak -m '{"text":"Hello","voice":"Daniel"}'
```

---

## Spinoff Details

### Why Spun Off
- Rust `target/` directory grew to 1.7GB
- Nat-s-Agents repo became bloated
- Project matured enough to graduate from incubation

### Spinoff Process (2026-01-04)
1. `gh repo create laris-co/voice-tray-v2 --private`
2. `rsync` code (excluding `target/`)
3. Push to new repo
4. Result: 1.7GB → 11MB

### Commit
```
7795f99a feat: spinoff voice-tray-v2 (1.7GB → 11MB own repo)
```

---

## Location History

| Phase | Path |
|-------|------|
| Incubate (v1) | `Nat-s-Agents/ψ/incubate/voice-tray/` (still exists) |
| Incubate (v2) | `Nat-s-Agents/ψ/incubate/voice-tray-v2/` (moved) |
| **Graduated** | `/Users/nat/Code/github.com/laris-co/voice-tray-v2/` |

---

## Where to Get More Info

### Oracle MCP
- **Thread 13**: "Voice Tray v2 Project Context" - full context dump
- Use: `mcp__oracle-v2__oracle_thread(threadId: 13, message: "your question")`

### claude-mem
- Search: `mcp__plugin_claude-mem_claude-mem-search__search(query: "voice-tray")`
- Recent: Check observations #5217-#5250 (Jan 2-3 work)

### Git History (Nat-s-Agents)
```bash
cd /Users/nat/Code/github.com/laris-co/Nat-s-Agents
git log --oneline --grep="voice-tray" | head -20
```

### Documentation Files
| File | Content |
|------|---------|
| `ψ/writing/drafts/2026-01-02_voice-tray-building-tauri-app-with-ai.md` | Blog draft |
| `ψ/writing/drafts/2026-01-03_tauri-tray-icon-antigravity-workflow.md` | Icon guide |
| `ψ/memory/retrospectives/2026-01/02/23.12_voice-tray-v2-complete.md` | Retrospective |
| `ψ/memory/learnings/2026-01-04_voice-tray-v2-spun-off*.md` | Spinoff record |

### GitHub
- Issue #92: `feat: Voice Tray v2 — MQTT Backend`
- Repo: `https://github.com/laris-co/voice-tray-v2`

---

## Bugs Fixed

1. **Duplicate Tray Icon** - Removed dual initialization in tauri.conf.json (Tauri issue #8982)
2. **White Square Icon** - Fixed SVG bounding box, converted to PNG32 with ImageMagick
3. **Multi-monitor Positioning** - Use physical coordinates for popup placement

---

## Key Patterns Learned

1. **Tauri 2.0 Tray**: Left vs right click handling, `LSUIElement` for no dock icon
2. **PNG32 Format**: Required for menu bar transparency on macOS
3. **ImageMagick**: `-fuzz 30% -transparent -trim -resize 22x22 PNG32:`
4. **Click Debounce**: 300ms to prevent double windows
5. **MQTT Retained**: Use for status messages that new subscribers need

---

## Incubation Lifecycle (Oracle Pattern)

```
🌱 Seed → 🌕 Grow → 🎓 Graduate → 🤝 Reunion
```

Voice Tray v2 is now at **🎓 Graduate** phase - living in its own repo while knowledge (retrospectives, learnings) stays in Nat-s-Agents.
