---
title: Code Review
slug: code-review
date: 2026-01-02
tags: [essays]
---

Code review is a collaborative process where engineers propose changes to the codebase and others evaluate those changes with one primary goal: improving the long-term health of the system. It is not a final exam, a gatekeeping ritual, or a search for perfection. Every pull request should aim to leave the codebase in a better state than before.

Reviews are iterative by nature. Feedback, revisions, and clarification are expected, and multiple rounds are normal. A review is complete when the reviewer is confident that the change improves the system overall and that its intent is clear to future readers.

Code review is also one of the most effective learning tools in engineering. It exposes both authors and reviewers to new patterns, trade-offs, and ways of thinking that rarely show up in isolated work.

## Why code review matters

Code review is how teams scale understanding. By reading and discussing changes, engineers build a shared mental model of how the system works and why it is designed the way it is. This shared understanding compounds over time and across teams.

Every merged pull request becomes part of the permanent history of the codebase. Titles, descriptions, and commit messages often become the primary source of context for future engineers trying to modify or remove code. Clear reviews today reduce confusion and risk tomorrow.

Confusion during review is a signal, not a failure. If a reviewer struggles to understand a change, it is likely that future engineers will struggle too. Improving clarity during review directly improves long-term maintainability.

## Core principles

- The goal of every review is to improve overall code health, even if only slightly.
- Readability and clarity matter more than cleverness or abstraction.
- Complexity is a cost; code that cannot be understood quickly is usually too complex.
- Progress beats perfection. Small, consistent improvements compound.
- Reviewer time is valuable and should be actively protected.

## Guidelines for authors

### Keep pull requests small

Smaller pull requests are easier to review, easier to reason about, and more likely to be merged quickly. Aim for one focused change per PR, including its relevant tests.

When working on larger features, split the work vertically so each PR represents a thin slice across layers if needed. Use feature flags to merge incremental or incomplete work safely.

If a change cannot be made small, consider doing a paving refactor first or ask other engineers for help identifying clean boundaries.

### Write for reviewers and future readers

A pull request should be understandable without reading the entire diff. Titles and descriptions should clearly explain what changed and why. Include enough context so a reviewer can orient themselves immediately.

Treat the final merged commit message as permanent documentation. Future engineers will rely on it to understand why the code exists.

Before submitting, review your own PR in GitHub as if you were the reviewer. Look for unnecessary noise, unclear intent, or confusing structure.

### Maintain good PR hygiene

Avoid mixing functional changes with refactors, formatting, or IDE-driven edits. Separate them so reviewers can focus on one type of change at a time.

Break work into clear stages—such as bug fixes, refactors, or follow-ups—and make it obvious which stage the current PR represents.

During review, respond promptly to feedback to reduce context switching for reviewers. Clearly signal when updates are pushed, ask for clarification when feedback is unclear, and defer to reviewer preferences on subjective matters. Reviewers represent future readers.

Always design PRs with empathy. Reviewers are collaborators, not QA systems.

## Guidelines for reviewers

### Review with the right mindset

Evaluate pull requests based on whether they improve the system overall, not whether they are perfect. Approve changes once they clearly move the codebase forward.

Use “nit” comments for low-priority suggestions that should not block progress. Focus first on high-level concerns like design, structure, and intent before diving into details.

### Give clear, principled feedback

Explain _why_ you are giving feedback, not just _what_ should change. Tie comments to shared engineering principles rather than personal preference.

Use neutral, collaborative language (“we” or passive voice) and support feedback with examples or suggested code when helpful. Acknowledge good decisions and clear thinking—positive feedback reinforces good habits.

### Know what to look for

Review changes end-to-end, following the flow of data and logic from inputs to user-visible behaviour. Watch for unnecessary generalisation, unused flexibility, or complexity that the system does not need.

Check that names communicate intent, responsibilities are clear, and documentation explains purpose and behaviour where appropriate. Look for natural boundaries where the PR could be split to reduce scope.

### Mental models and heuristics

Think of code quality in rough letter grades and aim to improve it by one grade per pull request. Not every change needs to be an A+, but every change should make the system better than it was before.

### Keep the process moving

Aim to complete each review round within one business day where possible. If written feedback is not converging, suggest a short call to unblock and align. Start reviews from a known entry point and follow the logical flow rather than jumping randomly through files.

## Shared responsibility

Code review is a joint effort between authors and reviewers to improve the system, not to win arguments or assert ownership. Both sides should optimise for learning, clarity, and long-term maintainability.

Minimise delays between review rounds to avoid repeated effort and lost context. End every PR with a clean, accurate title and description that clearly explain what exists and why.

## References

<https://google.github.io/eng-practices/>
<https://mtlynch.io/code-review-love/>
