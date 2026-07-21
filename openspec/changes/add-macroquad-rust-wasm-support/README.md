# Add Macroquad Rust/WASM Support

Status: complete

## Scope

Add Macroquad as a first-class lightweight Rust game stack for native and web
targets. The support surface includes engine selection, a reusable skill, a
specialist agent, version-aware reference documentation, and repository catalog
entries.

## Milestones

- M1 — Define and implement the Macroquad Rust/WASM support contract.
- M2 — Wire discovery surfaces and validate the resulting catalog.

## Validation Evidence

- `/usr/bin/python3 refenrece/skills/skill-creator/scripts/quick_validate.py
  refenrece/skills/macroquad-rust-wasm` — passed.
- `/usr/bin/python3 refenrece/skills/skill-creator/scripts/package_skill.py
  refenrece/skills/macroquad-rust-wasm /private/tmp/openagenticgame-skill-dist`
  — passed; package created outside the repository.
- `bash -n refenrece/skills/macroquad-rust-wasm/scripts/validate.sh` — passed.
- Manifest count and unique Macroquad entry checks with `jq` — passed.
- Macroquad 0.4.15 smoke fixture on Rust 1.97.1 — formatting, clippy,
  native tests, and release `wasm32-unknown-unknown` build passed through the
  reusable validation script.
- `git diff --check` — passed at closeout.

The smoke fixture exposed Rust 1.96+'s removal of WASM's implicit
`--allow-undefined` behavior. The validated compatibility flag is now part of
the script, specialist guidance, engine reference, and Codex validation matrix.

## Persona Review

- Security: the validation script quotes the supplied project path, performs no
  destructive operations, and recommends a locally pinned production loader.
- Performance: native checks plus a release WASM build are intentionally a
  heavier ship gate; no runtime overhead is added to adopting games.
- Readability: setup routing, specialist ownership, validation, and discovery
  names use one canonical `macroquad` / `macroquad-rust-wasm` vocabulary.
- Runtime regression: native and linker paths are proven, while real browser
  execution remains the explicit residual risk below.

## Known Limitations

- This repository is a studio template, not a concrete Cargo game. Compilation
  was verified with a generated fixture, but the HTML/loader bundle was not
  executed in a browser during this change.
- `shellcheck` was unavailable locally; Bash syntax and end-to-end execution
  were both verified instead.
