# Tasks

## M1 — Macroquad Support Contract

- [x] Create `macroquad-rust-wasm` skill using the repository skill scaffold.
- [x] Add the Macroquad programming specialist.
- [x] Add version-aware Macroquad engine reference documentation.
- [x] Extend `setup-engine` with Macroquad aliases, stack defaults, and checks.

Validation package:

- Skill frontmatter validation.
- Focused content checks for Rust, Macroquad, and WASM build commands.

Fallback: keep the new support isolated under its skill/reference directories if
the broader discovery docs cannot be updated coherently.

## M2 — Discovery And Catalog Sync

- [x] Add Macroquad to the skill manifest.
- [x] Add Macroquad to engine and agent discovery documentation.
- [x] Reconcile affected visible counts.
- [x] Record validation evidence and residual risk.

Validation package:

- JSON parse and manifest-count check.
- Repository-wide Macroquad discovery search.
- Git diff review and whitespace check.

Fallback: remove only the catalog entries if validation finds an unresolved
discovery mismatch; M1 remains independently usable by file path.
