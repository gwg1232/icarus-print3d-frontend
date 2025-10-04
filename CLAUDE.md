# Guidelines for Claude Code

## Tech Stack

- **Backend**: Axum (Rust web framework)
- **Database**: PostgreSQL with SQLx
- **Templates**: Maud (compile-time HTML templates)
- **Frontend**: HTMX + Tailwind CSS
- **Sessions**: tower-sessions with PostgreSQL store

### HTMX Usage Policy

- Prefer standard HTML forms and links when possible
- Use HTMX only when:
  - Standard HTML cannot accomplish the task
  - HTMX provides significantly better UX
  - Action requires non-standard HTTP methods on non-form elements

## Code Style

- Follow modern Rust conventions
- **Single standard principle**: Maintain exactly one way to accomplish each task - avoid creating multiple functions or patterns for the same purpose
- Minimize symbol visibility: prefer private unless public is required

## Naming

- Improve naming for files, functions, variables, and identifiers
- Use `snake_case` for URLs (with underscores)
- HTTP handler names follow the pattern: `method_endpoint`
  - Endpoint derived from route path: static segments as-is, path params as `{resource}_id`, joined with underscores
