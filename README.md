# marketplace

The `pragmatic-engineer` plugin marketplace for Claude Code. It lists one plugin, `playbook`.

The plugin itself lives in [pragmatic-engineer/playbook](https://github.com/pragmatic-engineer/playbook) under `plugins/playbook`, next to the `playbook` binary. This repository holds only the listing, this README and the checks on the listing.

## Install

```sh
claude plugin marketplace add pragmatic-engineer/marketplace
claude plugin install playbook@pragmatic-engineer
```

Claude Code v2.1.224 or later is recommended. The full install guide, including the binary and the `install.sh` script, is in the [playbook repository](https://github.com/pragmatic-engineer/playbook/blob/main/docs/guides/00-install.md).

## How the listing works

The `playbook` entry in `.claude-plugin/marketplace.json` is a `git-subdir` source: the url `https://github.com/pragmatic-engineer/playbook.git` and the path `plugins/playbook`. It sets no `ref` and no `version`. So the entry follows the default branch of the playbook repository, and Claude Code updates the plugin when the `version` in the plugin's own `plugin.json` changes. A change to the plugin needs no change here.

## Checks

```sh
shell/check-marketplace.sh
```

The script checks that every source is a `git-subdir` source over https with a safe relative path, that it sets no `ref`, `sha` or `version`, and that the manifest at that path on the default branch carries the same plugin name. Set `OFFLINE=1` to skip the lookup.

## History

Before the plugin moved into the playbook repository, it lived here. Its history stays readable in this repository, up to the release `playbook` 0.27.2.

## License

Apache-2.0. See `LICENSE` and `NOTICE`.
