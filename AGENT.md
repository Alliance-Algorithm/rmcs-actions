# Agent Context

## Required Workflow

1. **Show Conventional Commit messages**
   - For every change the agent makes, present a corresponding Conventional Commit message suggestion to the user.
   - Use standard Conventional Commit types (for example: `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`).

2. **Document new packages thoroughly**
   - Every newly added sub-package must include its own dedicated `README.md`.
   - All package code must be well and fully documented, including:
     - Exposed/public APIs
     - Internal implementation details

## Priority

If multiple instruction sources exist, follow direct user instructions first, then this file, unless another higher-priority policy explicitly overrides it.
