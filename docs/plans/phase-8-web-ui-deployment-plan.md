# Phase 8 Plan: Web UI + Deployment

## Objective

Deliver a modern, phone-friendly Sudoku experience while preserving the current Rust solver quality and test coverage.

Primary goals:

1. Keep core solver/game logic in Rust.
2. Build a modern React + TypeScript frontend.
3. Make the app accessible on Android (Pixel) by deploying it to the web.
4. Keep Tauri desktop support as optional follow-on, not blocker.

## Decision Summary

Recommended implementation path:

1. **Do not rewrite solver in Python.**
2. **Do not use a Python-first frontend.**
3. **Use Rust core + React UI + WebAssembly bridge** for fastest path to polished UX and low-cost deployment.
4. Start with **static hosting** (no server required) for simplest and cheapest first release.

Reasoning:

1. Existing Rust solver is already validated and fast.
2. React ecosystem is stronger for rich UI/animation and responsive layouts.
3. WASM keeps logic in Rust and removes backend hosting complexity for v1.
4. Static deploy is typically free on hobby tiers.

## Scope and Non-Goals

In scope:

1. Refactor/reuse Rust code for browser execution via WASM.
2. New responsive React UI with parity for core gameplay interactions.
3. Public web deployment and phone verification.

Out of scope for initial Phase 8:

1. Full account system / cloud saves.
2. Multiplayer features.
3. Rewriting solver in another language.
4. Mandatory mobile app store release.

## Target Architecture

1. `sudoku-core` (Rust crate)
    - Pure solver/game logic, no Tauri/runtime dependencies.
2. `sudoku-wasm` (Rust crate)
    - `wasm-bindgen` wrapper over `sudoku-core`.
    - Exposes browser-safe API for UI.
3. `src-web` (React + TypeScript + Vite)
    - New frontend that calls WASM exports.
4. Existing `src-tauri`
    - Keep operational.
    - Optional later: point Tauri UI to React bundle for single UI codebase.

## Proposed Repository Layout

```text
/
    src-tauri/                # existing Tauri backend + headless/test runner
    src-ui/                   # existing vanilla UI (legacy for now)
    src-web/                  # new React+TS web UI
    crates/
    sudoku-core/            # extracted Rust engine logic
    sudoku-wasm/            # wasm-bindgen layer for browser
    docs/plans/
    phase-8-web-ui-deployment-plan.md
```

## Milestones and Acceptance Criteria

### Phase 8A: Extract Core Rust Logic

Tasks:

1. Create `crates/sudoku-core`.
2. Move/port game+solver logic from `src-tauri/src/sudoku_game/` into core crate.
3. Keep deterministic APIs for:
    - set/delete value
    - clear grid
    - grid snapshot (for UI rendering)
    - solution counting with cap
4. Update tests to run against core crate.

Acceptance criteria:

1. Existing solver behavior matches current `src-tauri`.
2. Core crate unit tests pass.
3. Existing harness logic still passes (directly or via adapter).

Estimated effort: 0.5 to 1.5 days.

### Phase 8B: WASM Bridge

Tasks:

1. Create `crates/sudoku-wasm` with `wasm-bindgen`.
2. Export safe, minimal API:
    - `new_game()`
    - `set_value(row, col, value)`
    - `delete_value(row, col)`
    - `clear()`
    - `count_solutions()`
    - `get_grid_json()`
3. Build browser package with standard wasm tooling.

Acceptance criteria:

1. Browser can instantiate module and call APIs.
2. Boundary validation remains enforced server-side equivalent (inside Rust).
3. No panics for normal malformed UI input; return controlled error strings.

Estimated effort: 0.5 to 1.5 days.

### Phase 8C: React UI Baseline

Tasks:

1. Scaffold `src-web` with Vite + React + TypeScript.
2. Build responsive Sudoku grid + controls:
    - cell selection
    - number input (desktop + touch)
    - clear/count/debug-equivalent controls
3. Preserve UX semantics:
    - user-entered values styled differently from solver-filled values
    - solved/invalid states clear to users
4. Add loading/error UX for WASM init and command failures.

Acceptance criteria:

1. Desktop and mobile layouts work.
2. Core gameplay features from current UI are available.
3. Manual smoke test succeeds on local desktop and Pixel browser.

Estimated effort: 1 to 3 days.

### Phase 8D: Parity + Regression Coverage

Tasks:

1. Keep existing Rust unit/integration tests green.
2. Add minimal web UI tests:
    - basic render
    - input interaction
    - solved-cell color semantics
3. Add one E2E smoke test for key flow (open puzzle, enter value, see update).

Acceptance criteria:

1. Rust tests remain green.
2. UI tests pass in CI/local.
3. No behavioral regressions in known edge cases.

Estimated effort: 0.5 to 1.5 days.

### Phase 8E: Deploy for Phone Access

Tasks:

1. Deploy static web app to one provider (Cloudflare Pages, Vercel, or Netlify).
2. Configure build command and output dir.
3. Verify production URL on Pixel.
4. Optional: add PWA manifest for install-like behavior.

Acceptance criteria:

1. Public URL loads and is playable on Android Pixel.
2. No backend required for v1 deploy.
3. Deployment steps documented in `Readme.md` or dedicated doc.

Estimated effort: 0.5 day.

## Testing and Validation Gates

From repo root:

1. `cargo test --manifest-path src-tauri/Cargo.toml`
2. `cargo run --manifest-path src-tauri/Cargo.toml --bin test_runner -- test`
3. `cargo test` for `sudoku-core` (once created)
4. Web checks:
    - `npm run test` (or equivalent)
    - `npm run build`
    - `npm run preview` smoke check

## Deployment Cost Guidance (Hobby Scale)

### Initial recommended path (static hosting only)

Typical cost: **$0/month**.

Potential cost items:

1. Domain name (optional): approximately **$10-$20/year**.
2. Paid hosting tier only if traffic/features exceed free limits.

### If adding a backend API later

Typical starter cost: **$5-$20/month** for one small always-on service.

### Pricing pages (check current values directly)

1. Vercel: https://vercel.com/pricing
2. Netlify: https://www.netlify.com/pricing/
3. Cloudflare Pages: https://pages.cloudflare.com/
4. Render: https://render.com/pricing
5. Railway: https://railway.com/pricing
6. Fly.io: https://fly.io/pricing

## Risks and Mitigations

1. Risk: logic drift between Tauri and web paths.
    - Mitigation: centralize logic in `sudoku-core`, keep UI thin.
2. Risk: WASM packaging/tooling friction.
    - Mitigation: keep bridge crate minimal and API stable.
3. Risk: mobile UX regressions.
    - Mitigation: early Pixel testing before visual polish phase.
4. Risk: deployment confusion for first-time flow.
    - Mitigation: step-by-step deployment runbook in docs before release.

## Security and Reliability Notes

1. Preserve strict input validation at Rust API boundaries.
2. Return controlled user-facing errors; avoid panic paths.
3. Do not log secrets/tokens if deployment adds CI/CD secrets.
4. Keep dependencies minimal and pinned where practical.

## Resume Checklist (Work Tracking)

Use this section to continue work later without rediscovery.

- [ ] Phase 8A started
- [ ] Phase 8A complete
- [ ] Phase 8B started
- [ ] Phase 8B complete
- [ ] Phase 8C started
- [ ] Phase 8C complete
- [ ] Phase 8D started
- [ ] Phase 8D complete
- [ ] Phase 8E started
- [ ] Phase 8E complete

Current status notes:

1. Solver currently in Rust with exact-cover + DLX and passing harness tests.
2. Existing test harness reads `test/*.txt` and currently validates 10 puzzle files.
3. Next actionable step: begin Phase 8A (`sudoku-core` extraction).

## First Step When Resuming

1. Create `crates/sudoku-core` and move pure solver/game modules there.
2. Keep `src-tauri` compiling by adding a temporary adapter layer.
3. Re-run existing Rust test + harness commands from repo root before starting web UI work.
