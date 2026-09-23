# Secure Backup

[![CI](https://github.com/abdulwalidal/secure-backup/actions/workflows/ci.yml/badge.svg)](https://github.com/abdulwalidal/secure-backup/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Tauri 2](https://img.shields.io/badge/Tauri-2.0-24c8db.svg)](https://tauri.app/)

A lightweight, privacy-focused, zero-knowledge desktop backup application built with **Tauri 2**, **Rust**, and **Vanilla TypeScript**.

---

## Overview

**Secure Backup** gives you full control and privacy over your data. Backups are processed, fingerprinted with cryptographic hashes, and prepared for zero-knowledge client-side encryption before reaching any cloud storage provider.

```text
User selects folder
        ↓
Scan & collect metadata
        ↓
Streamed SHA-256 integrity hashing
        ↓
Compare changes (incremental backup)
        ↓
Client-side encryption (AES-256-GCM / ChaCha20)
        ↓
Upload encrypted ciphertext to cloud
        ↓
Record snapshot metadata in local SQLite
```

---

## Key Features

- **Lightweight Desktop Shell:** Native performance and minimal memory footprint powered by Tauri 2 (no bloated Electron dependencies).
- **Zero-Knowledge Architecture:** Original plaintext never leaves your machine unencrypted.
- **Cryptographic Hashing:** Buffered, streaming **SHA-256** checksum verification for files of any size without RAM bloat.
- **Structured Snapshot Engine:** Preserves file hierarchy, modification timestamps, and manifests.
- **Privacy First:** Zero analytics, telemetry, tracking, or unsolicited network requests.

---

## Project Architecture

```text
secure-backup/
├── src/                      # Frontend (TypeScript, HTML, CSS)
│   ├── index.html            # Native desktop UI shell
│   ├── styles.css            # Dark desktop theme & responsive styles
│   └── main.ts               # Tauri IPC invocations & reactive state
├── src-tauri/                # Backend Core (Rust)
│   ├── src/
│   │   ├── backup/           # Backup orchestrator & recursive scanner
│   │   ├── commands/         # Tauri command handlers (IPC boundary)
│   │   ├── hashing/          # Streaming SHA-256 hashing engine
│   │   ├── models/           # Manifest, file metadata, & result types
│   │   ├── lib.rs            # Application entry point & plugin registrations
│   │   └── main.rs           # Desktop runner
│   ├── capabilities/         # Tauri 2 least-privilege security permissions
│   ├── Cargo.toml            # Rust dependencies & optimization profiles
│   └── tauri.conf.json       # Desktop window & security configurations
├── .github/                  # CI/CD Workflows, PR templates, and issue forms
├── ARCHITECTURE.md           # Detailed security and system design
├── CONTRIBUTING.md           # Contribution guidelines & branching model
└── LICENSE                   # MIT License
```

---

## Getting Started

### Prerequisites

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libwebkit2gtk-4.1-dev
```

#### Node.js & Rust
- Node.js (v18+)
- Rust & Cargo (1.78+)

### Development

1. **Clone the repository:**
   ```bash
   git clone git@github.com:abdulwalidal/secure-backup.git
   cd secure-backup
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Run unit tests:**
   ```bash
   cargo test --manifest-path src-tauri/Cargo.toml
   ```

4. **Launch development desktop app:**
   ```bash
   npm run tauri dev
   ```

---

## Roadmap

- [x] **Milestone 1:** Tauri 2 desktop shell, modern UI, native GTK folder selection IPC.
- [x] **Milestone 2:** Recursive filesystem scanner, streaming SHA-256 hashing, structured backup engine.
- [ ] **Milestone 3:** Local SQLite database for persistent snapshot history and metadata.
- [ ] **Milestone 4:** Client-side authenticated encryption (AES-256-GCM / ChaCha20-Poly1305).
- [ ] **Milestone 5:** First cloud storage provider integration (S3-compatible / Backblaze B2).
- [ ] **Milestone 6:** Verified end-to-end file restoration flow.
- [ ] **Milestone 7:** Hash-based incremental change detection.

---

## Contributing

We welcome community contributions! Please review [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on our branching model, code standards, and PR process.

**Note:** Direct pushes to `main` are disabled. All changes must go through a feature branch and a reviewed Pull Request.

---

## Security

For vulnerability reporting and our security principles, see [SECURITY.md](SECURITY.md).

---

## License

This project is licensed under the [MIT License](LICENSE).
