# Crate `domain`

## Purpose

Library crate for agent domain logic: dialog session state, LLM provider
abstractions, and OpenAI-compatible message types. No I/O or UX.

Package: `domain` · check with: `cargo check -p domain`

## Responsibilities

**Does:**

- owns dialog `Session` state machine (`Idle` / `Generating`) and turn
  lifecycle (`begin_turn`, `complete_turn`, `cancel_turn`, `fail_turn`);
- defines the async `Provider` trait for completions with cancellation;
- holds OpenAI-compatible chat completion response structs.

**Does not:**

- talk to the network or call an LLM implementation;
- own CLI/TUI, persistence, or tool execution;
- enforce budgets or stop policy yet (planned at workspace level).

## Current State

Early domain scaffold:

- `session` — in-memory dialog history and request-id–guarded turns;
- `llm_provider` — `Provider` trait and error enum;
- `open_ai_compatible` — chat completion response types (no HTTP client);
- root exports `DialogSessionsError` and `CompletionResult`.

Unit tests cover `begin_turn`, `cancel_turn`, and `fail_turn`.

## Dependencies

Direct crate deps (not yet lifted into `[workspace.dependencies]`):

| Crate | Role |
|---|---|
| `async-trait` | async `Provider` trait |
| `serde` | serialization support |
| `tokio-util` | `CancellationToken` for completions |
| `uuid` (v4) | turn `request_id` |

Consumers: `cli` depends on this crate via `{ path = "../domain" }`.

## Key Decisions

- Session is request-id keyed: overlapping completions and mismatched
  complete/cancel/fail calls fail with `DialogSessionsError`.
- Provider is a trait in domain; concrete backends live elsewhere.
- OpenAI-compatible types live here as protocol shapes, not as a client.

## Commands

```bash
cargo check -p domain
cargo test -p domain
```
