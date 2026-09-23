# Architecture & Security Design

This document details the system design, boundaries, and security model of **Secure Backup**.

---

## 1. System Overview

```text
               +----------------------------------+
               |        Desktop UI (WebKit)       |
               |        TypeScript / HTML / CSS   |
               +-----------------+----------------+
                                 |
                     Tauri IPC Commands
                                 |
               +-----------------v----------------+
               |            Rust Core             |
               +-----------------+----------------+
                                 |
         +-----------------------+-----------------------+
         |                       |                       |
+--------v-------+      +--------v-------+      +--------v-------+
|  Backup Engine |      | Crypto/Hashing |      | Local Database |
| (Scan/Pack)    |      | (SHA-256/AES)  |      | (SQLite)       |
+----------------+      +----------------+      +----------------+
                                 |
                         Cloud Abstraction
                                 |
                        +--------v-------+
                        | Cloud Provider |
                        | (Encrypted)    |
                        +----------------+
```

---

## 2. Security Boundaries

### Tauri 2 Capability Model
- The frontend operates in an isolated webview.
- It cannot execute arbitrary shell commands or access the raw filesystem directly.
- All filesystem operations are mediated through explicit, strongly typed Tauri commands in `src-tauri/src/commands/mod.rs`.
- Capabilities are declared in `src-tauri/capabilities/default.json` following the principle of least privilege.

### Cryptographic Isolation
- Cryptographic hashing uses streaming SHA-256 (`sha2`).
- Encryption (Phase 7) will use standard authenticated encryption (`AEAD`: AES-256-GCM or ChaCha20-Poly1305).
- Encryption keys will never be stored in plaintext or logged.
- The cloud provider layer only ever handles encrypted ciphertext payloads.

### Privacy & Telemetry
- The application contains **zero analytics, tracking, or telemetry**.
- Outbound network requests only occur when a backup or restore is explicitly initiated by the user.
