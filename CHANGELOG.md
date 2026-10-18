# Changelog


## 1.5.8 — 3 November 2026

A maintenance release.

### Fixes

- Diff view scrolls correctly on small screens.

## 1.5.7 · 2026-11-01

- Diff view highlights moved parts, not just new ones.
- Rewind restores terrain edits together with parts.
- Faster diff on large places.
- Approval list scrolls without lag.

## 1.5.6 · 2026-10-30

First release.

- `baseplate` terminal agent for Roblox Studio.
- Talks to Studio through Studio's built-in MCP server. The Baseplate plugin adds turn recording, selection, place snapshots, test copies and screenshots.
- Place diff before every change, with ask-first, auto-apply and plan-only modes.
- `/rewind` to any earlier turn: one Studio undo step and one git commit per turn.
- Playtest loop with a proof report: Output before and after, what was checked, and a screenshot.
- Playtests run in a local test copy of the place, so live DataStores are never touched.
- Free local models through Ollama out of the box. Claude, GPT and Gemini work with your own key.
- Reads `BASEPLATE.md` for project rules.
