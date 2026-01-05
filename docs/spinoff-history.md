# Voice Tray V2 Spinoff History

Research conducted via claude-mem on 2026-01-04.

## 1. When Was Voice-Tray-V2 Spun Off from Nat-s-Agents?

**Date: January 4, 2026**

The spinoff occurred on January 4, 2026, as documented in multiple claude-mem observations:

- The project was previously housed at `/Users/nat/Code/github.com/laris-co/Nat-s-Agents/ψ/incubate/voice-tray-v2/`
- It was moved to its own repository at `/Users/nat/Code/github.com/laris-co/voice-tray-v2`
- The spinoff resulted in a dramatic size reduction from **1.7GB to 11MB** (the Nat-s-Agents monorepo was reduced from approximately 17GB to 11MB by removing voice-tray and other project files)

### Development Timeline

The voice notification system evolved through several phases:

| Date | Milestone |
|------|-----------|
| December 9, 2025 | Voice notifications built into Bridge Report and rrr command |
| December 9, 2025 | Extracted to claude-voice-notify as separate plugin repository |
| December 16, 2025 | Plugin approach deprecated |
| January 2, 2026 | Voice Tray v1 completed as Tauri desktop application |
| January 2, 2026 | Voice Tray v2 initiated with MQTT integration |
| January 4, 2026 | Voice Tray v2 spun off to standalone repository |

## 2. Why Did It Become Its Own Repository?

### Primary Reasons

1. **Repository Size Optimization**: The Nat-s-Agents monorepo had grown to 1.7GB+, with build artifacts, dependencies, and git history contributing to bloat. The spinoff reduced the monorepo to a lean 11MB.

2. **Project Maturity**: Voice Tray v2 "graduated" from the incubation system. The ψ/incubate/ directory in Nat-s-Agents serves as a staging area for experimental projects. Once a project proves viable and production-ready, it gets spun off.

3. **Independent Versioning**: As a critical, production-ready component, voice-tray-v2 deserved independent versioning and release cycles separate from the monorepo.

4. **Focused Development**: The standalone repository structure enables more focused development without the overhead of the larger Nat-s-Agents ecosystem.

### The Incubation System

The Nat-s-Agents repository implements a knowledge management workflow:
- `ψ/inbox/` - New items
- `ψ/incubate/` - Developing ideas and experimental projects (where voice-tray started)
- `ψ/active/` - Current work
- `ψ/lab/` - Experiments
- `ψ/memory/` - Observations and retrospectives
- `ψ/outbox/` - Completed items

Projects in incubate are symlinked to their canonical repository locations, allowing them to be developed within the agent environment while maintaining proper git structure.

## 3. The Spinoff Process

### ANTIGRAVITY Spinoff Pattern

The spinoff used the **ANTIGRAVITY spinoff pattern**, a systematic `/spinoff` command for extracting projects into standalone repositories. This was documented in commit `b9cca9a0`.

### Process Steps

1. **Create New Repository**: Using the `project-create.sh` script which:
   - Creates the GitHub repository under the laris-co organization
   - Clones it locally via ghq for structured codebase organization
   - Creates a symlink in the incubate directory
   - Registers the project slug

2. **Extract Project Files**: Move the implementation from `ψ/incubate/voice-tray-v2/` to the new repository

3. **Remove Files from Source Monorepo**: Voice-related files were deleted from Nat-s-Agents

4. **Document the Process**: Session retrospectives capture the spinoff for future reference

### Repository Structure After Spinoff

The new standalone repository at `github.com/laris-co/voice-tray-v2` contains:
- Tauri application configuration (`tauri.conf.json`)
- Rust source code (`main.rs`)
- Dependencies (`Cargo.toml`, `Cargo.lock`)
- Documentation (`README.md`)
- Notification scripts

### Technical Architecture

Voice Tray v2 is a Tauri-based desktop application featuring:
- **MQTT messaging** via rumqttc library
- **HTTP fallback server** via Axum framework
- **Multi-threaded voice queue processing**
- **Dynamic system tray UI** with custom facial expression icons (idle/speaking states)
- **Timeline UI** for viewing notification history

## Related Projects

The voice system is part of a larger ecosystem:
- **claude-voice-notify** - Claude plugin that sends notifications via MQTT
- **Jarvis** - Voice UI connected to Oracle v2 RAG
- **Arthur AI Assistant** - React components with voice improvements
- **Central Voice System** - Unified hub coordinating voice interfaces

## Sources

All information retrieved from claude-mem observations:
- #8477, #8479, #8489, #8495 - Voice tray documentation and spinoff details
- #8492 - Complete voice system development timeline
- #8532 - Spinoff confirmation
- #8556, #8557 - Voice tray v2 history and spinoff rationale
- #8174 - project-create.sh workflow documentation
- #8077 - ψ/ directory structure
