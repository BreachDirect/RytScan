name: Pull Request
description: Changes to RytScan
title: "[feat]: "
labels: []
body:
  - type: markdown
    attributes:
      value: |
        Thanks for contributing to RytScan! Please complete the checklist below.
  - type: textarea
    id: summary
    attributes:
      label: Summary
      description: What does this PR change and why?
    validations:
      required: true
  - type: input
    id: issue
    attributes:
      label: Linked issue
      description: e.g. `Closes #12`
  - type: textarea
    id: testing
    attributes:
      label: Testing performed
      description: |
        Commands run and results. CI runs `cargo fmt`, `cargo clippy -D warnings`, and `cargo test --workspace`.
    validations:
      required: true
  - type: checkboxes
    id: checklist
    attributes:
      label: PR checklist
      options:
        - label: `cargo fmt --all -- --check` passes
        - label: `cargo clippy --workspace --all-targets -- -D warnings` passes
        - label: `cargo test --workspace` passes
        - label: Tests included for new/changed behaviour
        - label: Docs updated (`docs/rules.md`, README, etc. as applicable)
