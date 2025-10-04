# Guidelines for Claude Code

## Code Style

- Follow modern Rust conventions
- **Single standard principle**: Maintain exactly one way to accomplish each task - avoid creating multiple functions or patterns for the same purpose
- Minimize symbol visibility: prefer private unless public is required

## Naming

- Improve naming for files, functions, variables, and identifiers
- Use `snake_case` for URLs (with underscores)
- HTTP handler names follow the pattern: `method_endpoint`
  - Endpoint derived from route path: static segments as-is, path params as `{resource}_id`, joined with underscores
