# Contributing

This repository holds the `playbook` Claude Code plugin (skills, commands, agents, output styles) and the `pragmatic-engineer` marketplace. The `playbook` binary and its hooks live in [pragmatic-engineer/playbook](https://github.com/pragmatic-engineer/playbook).

## Where things go

| You change | Edit |
|---|---|
| A skill, command, agent or output style | `plugins/playbook/` |
| The plugin version | `plugins/playbook/.claude-plugin/plugin.json` |
| The marketplace listing | `.claude-plugin/marketplace.json` |
| A guide for writing skills, commands or agents | `docs/authoring/` |
| The model and effort of a component | its frontmatter and the row in `docs/policy-table.md` |

## Checks

```sh
cd checks && cargo test --locked      # prose and structure checks
shell/check-marketplace.sh             # marketplace invariants
scripts/check-intents.sh [playbook]    # intents.json against a playbook binary
```

The plugin works without the binary. When a skill or command needs a tool the binary offers, it names the task in plain words and lists the intent in `intents.json`. Do not write a subcommand into the prose.

## Pull requests

- Conventional commit titles under 73 characters. The squash commit uses the title.
- Commits are signed and carry a `Signed-off-by` line.
- Keep a pull request under 1000 changed lines.
- Write plain English. No em or en dashes.

## Releases

Bump `version` in `plugins/playbook/.claude-plugin/plugin.json` in a pull request, merge it, then push a signed tag `vX.Y.Z` on the merge commit and create the GitHub release. The marketplace serves the plugin from this repository, so users update with `claude plugin update playbook@pragmatic-engineer`. The `playbook-hooks` entry is pinned by the `pin-hooks` workflow after each playbook release.
