# Contributing to Secure Backup

Thank you for your interest in contributing to **Secure Backup**! We are committed to building a secure, private, and lightweight desktop backup tool.

---

## Branching & Pull Request Model

To ensure security and stability, **direct pushes to `main` are strictly prohibited**.

All development follows this workflow:

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone git@github.com:<your-username>/secure-backup.git
   cd secure-backup
   ```
3. **Create a descriptive feature branch** off `develop` (or `main` if `develop` is absent):
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-description
   ```
4. **Make your changes** following our coding standards.
5. **Ensure all tests pass**:
   ```bash
   npm run build
   cargo test --manifest-path src-tauri/Cargo.toml
   cargo fmt --check --manifest-path src-tauri/Cargo.toml
   ```
6. **Commit with clear commit messages**:
   ```bash
   git commit -m "feat(hashing): add blake3 hash support"
   ```
7. **Push to your fork** and **open a Pull Request** against `develop`.

---

## Pull Request Guidelines

- Provide a clear summary of changes in the PR description.
- Reference any linked issues (e.g. `Closes #12`).
- Ensure all CI tests pass.
- Maintain documentation integrity: document any new Rust commands or public APIs.
- Keep PRs focused. Avoid combining multiple unrelated features into a single PR.

---

## Coding Standards

### Rust Core (`src-tauri/`)
- Follow standard Rust naming conventions and formatting (`cargo fmt`).
- Avoid `unwrap()` or `expect()` in production application code; return structured `Result` types.
- Never hard-code secrets, keys, or credentials.
- Do not log sensitive user data or unencrypted file contents.

### Frontend (`src/`)
- Use TypeScript with strict type checking enabled.
- Avoid introducing heavy frontend frameworks (React, Vue, etc.) without an approved RFC issue.
- Keep the UI lightweight, fast, and accessible.

---

## Code of Conduct

All contributors are expected to uphold the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
