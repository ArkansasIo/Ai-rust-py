# Security Policy Test Cases

These are explicit behavioral test cases for `rust_api/src/security.rs`. Convert them into executable unit tests in the Rust module as the test harness evolves.

| Action | Policy setting | Approval | Expected |
|---|---|---:|---|
| `shell.exec` | `allow_shell=false` | No | Deny |
| `shell.exec` | `allow_shell=true` | No | Allow unless another policy denies it |
| `network.http` | `allow_network=false` | No | Deny |
| `network.http` | `allow_network=true` | No | Allow unless another policy denies it |
| `filesystem.write` | `require_approval_for_writes=true` | No | Deny |
| `filesystem.write` | `require_approval_for_writes=true` | Yes | Allow unless another policy denies it |
| `git.commit` | `require_approval_for_writes=true` | No | Deny |
| `git.commit` | `require_approval_for_writes=true` | Yes | Allow unless another policy denies it |
| `filesystem.read` | default policy | No | Allow |
| `tool.custom` | default policy | No | Allow unless restricted by a higher-level allowlist |

## Additional invariants

1. Default policy must deny shell execution and network tools.
2. Approval must come from a trusted application/user workflow, not from the model response itself.
3. Authorization checks must be performed at the execution boundary immediately before side effects.
4. A tool-call budget must be enforced by the runtime; a `max_tool_calls` field alone does not enforce a limit.
5. Tool names and arguments should be validated against registered schemas before execution.
6. Denial responses should not reveal secrets or internal environment values.
7. Tests should cover exact action names and prefixes, including malformed or unexpected names.

## Important implementation note

These cases document intended behavior rather than asserting the current implementation already passes them. Tests should be implemented and run as part of CI before treating the policy as production-ready.
