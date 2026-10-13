# skills

The Claude Code side of [playbook](https://github.com/pragmatic-engineer/playbook): the `playbook` plugin (skills, slash commands and subagents) and the `pragmatic-engineer` plugin marketplace that serves it.

The plugin works on its own. The `playbook` binary and its hooks live in the playbook repository and ship as the second plugin, `playbook-hooks`.

## Install

```sh
claude plugin marketplace add pragmatic-engineer/skills
claude plugin install playbook@pragmatic-engineer
claude plugin install playbook-hooks@pragmatic-engineer
```

Claude Code v2.1.224 or later is recommended. The second install line is optional. The full install guide, including the binary and the `install.sh` script, is in the [playbook repository](https://github.com/pragmatic-engineer/playbook/blob/main/docs/guides/00-install.md).

## Layout

| Path | What it holds |
|---|---|
| `plugins/playbook/skills/` | Skills, one directory each |
| `plugins/playbook/commands/` | The `/playbook:*` slash commands |
| `plugins/playbook/agents/` | Subagent definitions |
| `plugins/playbook/output-styles/` | Output styles |
| `plugins/playbook/.claude-plugin/plugin.json` | The plugin manifest (name `playbook`) |
| `.claude-plugin/marketplace.json` | The marketplace listing both plugins |
| `checks/` | A small Rust crate that tests the prose and structure of the plugin |
| `shell/` | Scripts that guard and update the marketplace file |

## Checks

```sh
cd checks && cargo test --locked
shell/check-marketplace.sh
```

`shell/check-marketplace.sh` keeps installs keyless: a plugin source is either a path in this repository, or an https url or archive, and an archive carries a sha256.

## The hooks plugin pin

`playbook-hooks` is versioned with the playbook binary. Its entry points at the release archive `playbook-hooks-plugin-<version>.zip` in `pragmatic-engineer/playbook` and pins its sha256. The `pin-hooks` workflow runs daily at 06:37 UTC and can be started by hand. If a newer playbook release carries the archive, it runs `shell/bump-marketplace.sh`, which downloads the archive, verifies its build provenance attestation and writes the URL and the sha256 of those exact bytes. The workflow commits the change to a `bump/hooks-v<version>` branch through the GitHub API, opens a pull request, approves the held checks run, waits for it to pass and squash merges. It uses only the built-in `GITHUB_TOKEN`. Don't edit the pin by hand.

## History

The plugin files carry their history from the playbook repository. Commit hashes before the import differ from the ones in playbook because the history was rewritten to keep only these paths.

## License

Apache-2.0. See `LICENSE` and `NOTICE`.
