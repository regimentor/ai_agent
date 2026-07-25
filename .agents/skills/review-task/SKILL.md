---
name: review-task
description: >-
  Reviews a task description for clarity and completeness, then asks
  clarifying questions about anything ambiguous or missing. Use when the
  user pastes a task brief, asks to review a task, or invokes /review-task.
---

# Review Task

Review the task description and ask clarifying questions when something is
unclear. Do not implement anything or propose patches — review the brief only.

Reply to the user in Russian.

## Workflow

1. Read the full description. Do not invent missing details.
2. Check the text against the clarity checklist below.
3. If the brief is clear enough to execute — confirm briefly
   (1–3 sentences: goal, scope, done criteria) and stop.
4. If there are gaps — ask questions. Do not start work on assumptions.

## Clarity Checklist

Flag anything missing or ambiguous:

- **Goal**: what result is needed and why
- **Scope**: what is in and explicitly out
- **Done criteria**: how to know the task is finished
- **Context**: affected files/crates/systems (if applicable)
- **Constraints**: compatibility, style, bans, budgets, deadlines
- **Dependencies**: what already exists, blockers, prerequisites
- **Priority / order**: what must happen now vs later
- **Examples / contracts**: I/O, format, UX, API (when relevant)

## How to Ask Questions

- Only ask about real gaps; do not re-ask what is already stated.
- One question per uncertainty.
- Numbered list; critical blockers first, then the rest.
- If 2–3 reasonable options exist — list them briefly and ask which to pick.
- Do not turn the review into a full solution design — only what is required
  to execute the task correctly.

## Response Format

When clarifications are needed:

```markdown
## Понимание
[1–2 sentences: what is already clear]

## Вопросы
1. ...
2. ...
```

When the description is sufficient:

```markdown
## Готово к работе
[Brief summary: goal, scope, done criteria]
```

## After Answers

When the user replies — re-read the original brief together with the answers.
If gaps remain — ask another short round of questions.
If none remain — emit `## Готово к работе` and stop.
