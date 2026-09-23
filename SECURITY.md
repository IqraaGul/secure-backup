# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| < 0.2.0 | :x:                |

---

## Reporting a Vulnerability

Security is a primary concern for **Secure Backup**.

If you discover a security vulnerability, **please DO NOT open a public issue**. 

Instead, report it privately using one of the following methods:

1. **GitHub Security Advisory:** Submit an advisory privately via GitHub's [Security Advisories](https://github.com/abdulwalidal/secure-backup/security/advisories/new) page.
2. **Direct Contact:** Contact the repository maintainer directly via GitHub profile details.

### What to Include in Your Report
- A description of the vulnerability.
- Steps to reproduce or proof-of-concept code.
- Potential impact and affected components.
- Any suggestions for remediation.

We will acknowledge your report within 48 hours and work with you to resolve the issue responsibly before any public disclosure.

---

## Core Security Guarantees
- All cryptographic hashing (SHA-256) and future encryption (AES-256-GCM / ChaCha20) happen strictly client-side.
- Zero plaintext data or keys are transmitted to cloud storage.
- No telemetry, analytics, or background tracking.
