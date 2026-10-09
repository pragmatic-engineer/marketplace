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

The playbook entry points at the release archive `playbook-plugin-<version>.zip` and pins its sha256. The `bump` workflow runs daily at 06:37 UTC and can be started by hand. If `pragmatic-engineer/playbook` has a newer release, it runs `shell/bump-marketplace.sh`. The script downloads the archive, verifies its build provenance attestation, and writes the URL and the sha256 of those exact bytes. The workflow then runs `shell/check-marketplace.sh` and commits the change to a `bump/v<version>` branch through the GitHub API and opens a pull request. It uses only the built-in `GITHUB_TOKEN`, so no secret is stored. GitHub signs the commit, so it shows as verified. Events from `GITHUB_TOKEN` do not start other workflows, so the bump dispatches the `checks` workflow on the branch itself, waits for it to pass, and then squash merges the pull request through the API, which `protect-main` allows because the required checks passed on the branch head. Don't edit the pin by hand.

## License

Apache-2.0. See `LICENSE`.
