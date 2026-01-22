# Contributing to GitSensei

We love contributions! To keep the project high-quality and the commit history readable, we follow strict standards.

## 📜 Conventional Commits

GitSensei uses [Conventional Commits](https://www.conventionalcommits.org/). This isn't just a suggestion—it's what the tool itself is built on!

Please structure your commit messages as follows:

- `feat:` A new feature
- `fix:` A bug fix
- `docs:` Documentation changes
- `style:` Formatting, missing semi colons, etc; no code change
- `refactor:` Refactoring production code
- `test:` Adding missing tests, refactoring tests; no production code change
- `chore:` Updating build tasks, package manager configs, etc; no production code change

**Example:**
`feat(bundler): add AST-aware variable remapping`

## 🛠️ Development Workflow

1. **Fork** the repository.
2. **Create a branch** for your feature: `git checkout -b feat/my-cool-feature`.
3. **Write Tests:** If you are adding logic to the `bundler` or `llm` modules, please add a corresponding test in `tests/`.
4. **Lint your code:** Run `cargo fmt` and `cargo clippy`.
5. **Submit a PR:** Ensure your PR description clearly explains the change.

## 🧪 Testing

Run the suite to ensure no regressions in the minification logic:

```bash
cargo test
```

## 🛡️ Security

If you find a security vulnerability (e.g., a bypass in the PII redactor), please do not open a public issue. Email the maintainer directly at guycohen@example.com.