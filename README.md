# Arenic

**Arenic** is a solo  game inspired by large-scale MMORPG boss battles, Rabbit & Steal, Battle Royales, and Vampire Survivors. Built in **Rust** using the **Bevy** game framework, Arenic aims to capture the thrilling feel of 40-person raids—yet you can play it entirely **solo**, thanks to an **asynchronous Record & Replay** system.

---

## Table of Contents
1. [Project Overview](#project-overview)
2. [Gameplay Highlights](#gameplay-highlights)
3. [Etymology](#etymology)
4. [Design & Fonts](#design--fonts)
5. [Tech Stack](#tech-stack)
6. [Setup & Running](#setup--running)
7. [Development Tools & Commands](#development-tools--commands)
8. [Contact & Social](#contact--social)

---

## Project Overview
Arenic is all about intense arena battles and strategic layering of your heroes’ timelines. You can:
- **Select Classes**: Choose from multiple hero classes like Hunter, Alchemist, Sprinter, Gatherer, Thief, Tank, Cardinal, Merchant, and Bard—each with unique abilities.
- **Equip Gear**: Collect and upgrade gear with gacha-like loot and random drops.
- **Battle in Arenas**: Up to 8 distinct arenas, each with a 2-minute loop that can feature up to 40 characters (or “ghosts”).
- **Idle Progression**: Even when offline or not actively controlling a hero, the game continues simulating fights and resource gains.
- **Last Man Standing**: Survive the entire raid scenario to earn the biggest rewards.

---

## Gameplay Highlights
- **Record & Replay**: Each hero’s movements and actions are recorded in a 2-minute cycle. Once finalized, that timeline replays automatically, creating the feel of a massive 40-player raid—even if you’re playing alone.
- **Deterministic Raids**: Boss attacks, movement, and environment hazards follow strict timelines for perfect reproducibility. Future expansions may add subtle RNG after we fine-tune the core deterministic loop.
- **Solo**: The core is designed for single-player layering, but we’ll explore co-op options post-launch.
- **Gacha Recruitment**: Each arena can yield new recruits aligned with its theme (e.g., the Thief’s Arena spawns Thief-class recruits). Manage them in your Guild House and assign them to different arenas.
- **Idle System**: The game simulates progression when you’re offline—heroes can level up or even die from hazards.

---

## Etymology
From **arena** (sand, enclosed area) + **-ic** suffix, meaning “of or relating to an arena,” culminating in **Arenic**.
- **Adjective**: *Arenic*, comparative *more arenic*, superlative *most arenic*.

---

## Design & Fonts
- **Design Prototypes**: We use Figma for UI prototypes and quick iteration.
- **Fonts**:
    - *Migra* from PanagramPanagram
    - *Neue Montreal* from PanagramPanagram

---

## Tech Stack
- **Language**: [Rust](https://www.rust-lang.org/)
- **Game Engine**: [Bevy](https://bevyengine.org/)
- **Web Builds**: WebAssembly (wasm32-unknown-unknown) + [Trunk](https://trunkrs.dev/) for dev serving
- **YouTrack**: Tracking tasks and sprints at [YouTrack Cloud](https://stealth-startup.youtrack.cloud/agiles/177-4/current)

---

## Setup & Running
1. **Clone the Repo**
   ```bash
   git clone https://github.com/your-username/arenic.git
   cd arenic
   ```
2. **Build & Run (Native)**
   ```bash
   cargo run
   ```
3. **Build & Run (WebAssembly)**
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   trunk serve
   ```
4. **Watch & Auto-Reload**
   ```bash
   cargo watch -w src -w assets -i target -i .git -x 'run'
   ```

---

## Development Tools & Commands
- **Ensure you're on nightly**
```bash
rustc --version
rustc 1.84.0-nightly (03ee48451 2024-11-18)
```
- **Calculate Lines of Code (Daily)**:
  ```bash
  git log --since=midnight --diff-filter=A --name-only --pretty=format: | sort -u | xargs cloc
  ```
- **Live Reload**:
  ```bash
  cargo watch -w src -w assets -i target -i .git -x 'run'
  ```
- **Build & Serve WASM**:
  ```bash
  cargo build --release --target wasm32-unknown-unknown
  trunk serve
  ```
- **Task Management**:  
  We use [YouTrack](https://stealth-startup.youtrack.cloud/agiles/177-4/current) for issues & agile boards.

---

## Contact & Social
**Developer**: *Morning Harwood*
- [LinkedIn](https://www.linkedin.com/in/morningharwood)

> **Note**: This is my first game—please be gentle! Feedback and pull requests are welcome.

---

### Thanks for Checking Out Arenic!
Stay tuned for more features, boss battles, new classes, and behind-the-scenes dev updates. If you’d like to contribute or have suggestions, feel free to open an issue or start a discussion.

---

*Happy raiding, looting, and ghost-recording!*  