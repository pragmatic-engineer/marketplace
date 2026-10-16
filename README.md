# marketplace

The `pragmatic-engineer` plugin marketplace for Claude Code. It lists one plugin, `playbook`.

The plugin itself lives in [pragmatic-engineer/playbook](https://github.com/pragmatic-engineer/playbook) under `plugins/playbook`, next to the `playbook` binary. This repository holds only the listing, this README and the checks on the listing.

## Install

```sh
claude plugin marketplace add pragmatic-engineer/marketplace
claude plugin install playbook@pragmatic-engineer
```

The plugin works on its own. The `playbook` binary is optional and adds the hooks and the launcher. The recommended way to install it is the install script, which checks the SHA256 checksum of the release binary:

```sh
curl -fsSL https://raw.githubusercontent.com/pragmatic-engineer/playbook/main/install.sh | bash
playbook setup
```

Homebrew works too: `brew install pragmatic-engineer/tap/playbook`.

Claude Code v2.1.224 or later is recommended. The full install guide is in the [playbook repository](https://github.com/pragmatic-engineer/playbook/blob/main/docs/guides/00-install.md).

## How the listing works

The `playbook` entry in `.claude-plugin/marketplace.json` is a `git-subdir` source: the url `https://github.com/pragmatic-engineer/playbook.git` and the path `plugins/playbook`. Its `ref` pins a playbook release tag `vX.Y.Z`, and `metadata.version` carries the same version. The binary and the plugin share one version line, so a pinned plugin always matches a released binary.

The playbook release workflow moves the pin. After it publishes and verifies `vX.Y.Z`, it opens a `release/vX.Y.Z` pull request here from the `igorjs[bot]` app and turns on auto-merge. When that pull request merges, the `release` workflow tags the merge commit `vX.Y.Z` and publishes a release with the notes `playbook vA -> vB`. It acts only on a merged pull request from the app on a `release/v*` branch. It mints an app token from the organization secrets `BOT_APP_CLIENT_ID` and `BOT_APP_PRIVATE_KEY`, since the tag ruleset lets only the app create `v` tags. Without those secrets it stops with a notice, and the playbook release workflow tags and releases through the API instead.

To roll back the plugin, open a pull request that sets the `ref` back to the previous release tag `vA.B.C` and `metadata.version` to `A.B.C`, and run `shell/check-marketplace.sh` before you push. The tag and release of the bad version stay as they are.

## Checks

```sh
shell/check-marketplace.sh
```

The script checks that every source is a `git-subdir` source over https with a safe relative path, that it sets no `sha` or `version`, and that a `ref`, when set, is a release tag `vX.Y.Z`. It checks that the manifest at that path, at the `ref` or else on the default branch, carries the same plugin name, and the tag's version when a `ref` is set. `metadata.version`, when set, must be `MAJOR.MINOR.PATCH` and match the `playbook` ref. Set `OFFLINE=1` to skip the lookup.

## History

Before the plugin moved into the playbook repository, it lived here. Its history stays readable in this repository, up to the release `playbook` 0.27.2.

## License

Apache-2.0. See `LICENSE` and `NOTICE`.
