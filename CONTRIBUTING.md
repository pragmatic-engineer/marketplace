# Contributing

This repository holds the `pragmatic-engineer` marketplace listing. The `playbook` plugin, its docs and its checks live in [pragmatic-engineer/playbook](https://github.com/pragmatic-engineer/playbook), under `plugins/playbook`. Send plugin changes there.

## Where things go

| You change | Edit |
|---|---|
| The marketplace listing | `.claude-plugin/marketplace.json` |
| The check on the listing | `shell/check-marketplace.sh` |
| A skill, command, agent or output style | `plugins/playbook/` in the playbook repository |

## Checks

```sh
shell/check-marketplace.sh
```

## Pull requests

- Conventional commit titles under 73 characters. The squash commit uses the title.
- Commits are signed and carry a `Signed-off-by` line.
- Keep a pull request under 1000 changed lines.
- Write plain English. No em or en dashes.

## Releases

The marketplace has no releases. Users get a plugin update when the `version` in the plugin's `plugin.json` changes in the playbook repository. Do not add a `ref` or a `version` to the entry.
