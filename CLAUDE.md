# Guidelines for Claude Code

## Code Style

- Follow modern Rust conventions
- Maintain a single standard to keep the codebase compact and maintainable
- Minimize symbol visibility: prefer private unless public is required

## Naming

- Improve naming for files, functions, variables, and identifiers
- Use `snake_case` for URLs (with underscores)
- HTTP handler names follow the pattern: `method_endpoint`
  - Endpoint derived from route path: static segments as-is, path params as `{resource}_id`, joined with underscores
