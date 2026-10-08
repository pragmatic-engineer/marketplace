# pragmatic-engineer marketplace

Claude Code plugin marketplace for pragmatic-engineer.

## Plugins

- **playbook**: a pragmatic Claude Code toolkit (skills, slash commands, subagents, safety and state hooks). Source: [pragmatic-engineer/playbook](https://github.com/pragmatic-engineer/playbook).

## Use it

```bash
claude plugin marketplace add pragmatic-engineer/marketplace
claude plugin install playbook@pragmatic-engineer
```

The plugin installs over HTTPS, so no SSH keys are needed.

## Keeping the pin current

The playbook entry points at the release archive `playbook-plugin-<version>.zip` and pins its sha256. The `bump` workflow runs daily at 06:37 UTC and can be started by hand. If `pragmatic-engineer/playbook` has a newer release, it runs `shell/bump-marketplace.sh`. The script downloads the archive, verifies its build provenance attestation, and writes the URL and the sha256 of those exact bytes. The workflow then runs `shell/check-marketplace.sh` and pushes the commit to `main` with the `BUMP_TOKEN` secret, a fine-grained token with `contents: write` on this repo. `main` is protected, so the secret must belong to an admin. Don't edit the pin by hand.

## License

Apache-2.0. See `LICENSE`.
