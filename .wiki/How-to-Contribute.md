# How to Contribute

Thank you for your interest in contributing to UniLang. This page covers the workflow, conventions, and standards for contributing.

---

## Contribution Workflow

```
1. Pick an issue (or create one)
       |
2. Comment on the issue to claim it
       |
3. Fork the repository on GitHub
       |
4. Create a feature branch
       |   git checkout -b feature/my-feature
       |
5. Make your changes
       |
6. Write/update tests
       |
7. Run `make ci` locally (fmt + lint + test + build)
       |
8. Commit with a conventional message
       |
9. Push and create a Pull Request
       |
10. Address review feedback
       |
11. Merge (maintainer)
```

---

## Finding Issues to Work On

1. Check [GitHub Issues](https://github.com/hiteshkumar2187/unilang/issues) labeled `good-first-issue`.
2. Check the [Roadmap](https://github.com/hiteshkumar2187/unilang/blob/main/docs/planning/ROADMAP.md) for TODO items.
3. Run the test suite and look for gaps in coverage.
4. Try writing UniLang programs and report issues you find.

### Types of Contributions

| Type | Good First Issue? |
|------|-------------------|
| Bug fix | Often yes |
| Documentation | Yes |
| Test coverage | Yes |
| Tooling improvements | Sometimes |
| New feature | Usually no |
| Performance improvement | No |
| RFC (design proposal) | No |

---

## Commit Message Conventions

UniLang uses [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `test` | Adding or updating tests |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `ci` | CI/CD changes |
| `build` | Build system changes |
| `chore` | Other changes |

### Scopes

| Scope | Component |
|-------|-----------|
| `lexer` | Lexer/tokenizer |
| `parser` | Parser/AST |
| `semantic` | Semantic analyzer |
| `codegen` | Code generation |
| `runtime` | Runtime/bridge |
| `cli` | CLI tool |
| `vscode` | VS Code extension |
| `spec` | Language specification |

### Examples

```
feat(lexer): add f-string tokenization support
fix(parser): handle mixed indent/brace blocks correctly
docs(spec): clarify semicolon insertion rules
test(semantic): add cross-VM type inference tests
perf(codegen): optimize bridge call stub generation
```

---

## Testing Requirements

Every PR must include tests for the changes it introduces.

```
Unit tests:        Test individual functions and modules
Integration tests: Test compiler phases working together
End-to-end tests:  Test full compilation + execution of .uniL programs
Snapshot tests:    AST/UIR output stability
Benchmarks:        Track performance over time
```

Run tests locally before submitting:

```bash
make test              # Run all tests
make test-lexer        # Run specific test suite
make test-parser
make test-integration
```

---

## Code Style

### Rust (Compiler & Runtime)

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` with the project's `.rustfmt.toml` configuration
- Use `clippy` with no warnings: `cargo clippy -- -D warnings`
- All public APIs must have doc comments (`///`)
- Prefer `Result<T, E>` over panics
- Test coverage target: >90% for compiler phases

### Formatting and Linting

```bash
make fmt     # Format code
make lint    # Run linter
make ci      # Full check: fmt + lint + test + build
```

---

## Pull Request Process

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass (`make test`)
- [ ] Code is formatted (`make fmt`)
- [ ] Linter passes (`make lint`)
- [ ] New code has tests
- [ ] Documentation is updated (if applicable)
- [ ] Commit messages follow conventions
- [ ] PR description explains the "why"

### PR Template

```markdown
## Summary
Brief description of what this PR does and why.

## Changes
- Bullet list of specific changes

## Testing
How was this tested? What tests were added?

## Related Issues
Closes #123
```

### Review Process

- All PRs require at least **2 approvals** from maintainers
- CI must pass (build, test, lint, fmt)
- No unresolved conversations
- Maintainer merges via **squash merge**

### Review Timelines

| PR Size | Target Review Time |
|---------|--------------------|
| Small (< 100 LOC) | 2 business days |
| Medium (100-500 LOC) | 5 business days |
| Large (> 500 LOC) | 10 business days |

---

## RFC Process

Major changes to the language, compiler, or runtime require a Request for Comments (RFC):

1. Create an RFC document in `docs/rfcs/NNNN-title.md`
2. Open a GitHub Discussion tagged `rfc`
3. Community review period: minimum 2 weeks
4. Decision requires 3 maintainer approvals
5. Reference the RFC in the implementing PR

---

## License

By contributing to UniLang, you agree that your contributions will be licensed under the Apache License 2.0.

All source files must include the Apache license header:

```
// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.
```

---

**Previous**: [[Building Your First Model]] | **Next**: [[Development Setup]]
