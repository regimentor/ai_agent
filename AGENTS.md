# Agent Instructions

## Required in Every New Chat

1. **Immediately** read and follow the `project-info` skill:
   - path: `.agents/skills/project-info/SKILL.md`
   - or invoke `/project-info`
2. Do not start working on the repository until the skill has been loaded into context.
3. After loading the skill, read `crates/<name>/AGENT.md` when working on a specific crate.

This file (`AGENTS.md`) is an always-on repository rule. The `project-info`
skill is the canonical living project memory; do not duplicate it here. When
asked for a pre-commit memory update, follow its memory synchronization workflow
before committing.
