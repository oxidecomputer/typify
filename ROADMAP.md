# Typify Fork — Roadmap

Forked from [oxidecomputer/typify](https://github.com/oxidecomputer/typify). This roadmap tracks upstream PRs/issues we plan to integrate or fix before building on top of the fork.

## Phase 1: Quick Wins

Small, well-scoped fixes that improve correctness immediately.

- [x] **Cherry-pick PR #991** — Fix untagged enum automatic ordering. Integer variant must come before Number to avoid unreachable deserialization.
- [x] **#918** — Types with explicit defaults on required fields should implement `Default`.
- [x] **#986** — Bounded integer constructors ignore bounds (`From<u8>` instead of `TryFrom<u8>`).
- [x] **#843** — Integer `minimum` property rendered as float in generated code.
- [x] **#948** — Special characters in enum variant names (`=`, `>`, `<`, etc.) cause panics.

## Phase 2: Core `anyOf` Overhaul (Highest Impact)

The single highest-impact change — root cause of 5+ open issues.

- [x] **#414** — Replace broken `anyOf` flattened struct with delegation to `convert_one_of` (untagged enum).
  - Resolves: #895 (flatten on String panic), #710 (anyOf can't serialize), #790 (unreachable code in defaults.rs)
  - Partially resolves: #669 (wrong enum variants — now enum instead of broken struct)
  - Not resolved: #897 (allOf of oneOfs — separate merge.rs issue)

## Phase 3: `not` and `if/then/else` Support

- [x] **#480** — Transform `if/then/else` into `oneOf` via `allOf(if,then)` / `allOf(not(if),else)` in merge.rs.
- [x] **#847 / #489 / #954** — Replace panics in `convert_not` and merge.rs with graceful fallback to `serde_json::Value`.
- [x] **#435** — Replace `todo!()` panics in merge.rs `not` handling with best-effort behavior.

## Phase 4: Reference Handling (Architectural)

Major rework following upstream's "Big Plan" (#579).

- [ ] **#579** — External references, non-`$defs` references, JSON Schema 2020-12 support.
  - Unblocks: #201 (external `$ref` — most requested feature), #828, #299, #933, #955

## Phase 5: Polish

- [ ] **#862** — String enums with `const` values + fallback variant.
- [x] **#975** — Smallest integer type selection for bounded ranges (e.g. `[1..32]` → `NonZeroU8`).
- [x] **#882** — Replace patternProperties/propertyNames assertion panics with best-effort behavior.
- [ ] **#498** — Discriminator / internally tagged enum support.
- [ ] **#821 / #695 / #801** — Naming conflicts and `title` propagation.
- [ ] **#886** — schemars 1.0 upgrade.

## Upstream PRs Being Monitored

| PR | Title | Status |
|----|-------|--------|
| #982 | Fix allOf of oneOfs via cartesian product | Active review, promising |
| #987 | Handle bounded integers (DRAFT) | CI passes, direction may change |
| #912 | Better subtype naming from SchemaObject | Stalled, needs tests |

## Upstream PRs Skipped

| PR | Reason |
|----|--------|
| #988 | Maintainer flagged flawed logic |
| #939 | Upstream CI infra only |
| #938 | Upstream CI policy only |
| #293 | 3+ years old WIP, merge conflicts |
