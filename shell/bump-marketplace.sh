#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Igor Santos
# SPDX-License-Identifier: Apache-2.0
#
# bump-marketplace.sh: pin the playbook-hooks plugin to a playbook release.
# The entry is created on the first run and updated after that.
# Usage: bump-marketplace.sh <version-without-v> [marketplace-json]
# Downloads playbook-plugin-<version>.zip, verifies its build provenance
# attestation, then writes the release URL and the sha256 of those exact bytes
# into the playbook entry. Requires gh, jq and shasum.
set -euo pipefail

version="${1:?usage: bump-marketplace.sh <version> [marketplace-json]}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkt="${2:-$ROOT/.claude-plugin/marketplace.json}"
repo="pragmatic-engineer/playbook"
asset="playbook-hooks-plugin-${version}.zip"

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "::error::refusing unexpected version '${version}'" >&2
  exit 1
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

gh release download "v${version}" --repo "$repo" --pattern "$asset" --dir "$work"
gh attestation verify "$work/$asset" --repo "$repo" >/dev/null

sha="$(shasum -a 256 "$work/$asset" | cut -d' ' -f1)"
if [[ ! "$sha" =~ ^[0-9a-f]{64}$ ]]; then
  echo "::error::could not compute a sha256 for ${asset}" >&2
  exit 1
fi

url="https://github.com/${repo}/releases/download/v${version}/${asset}"
jq --arg url "$url" --arg sha "$sha" '
  def entry: {
    name: "playbook-hooks",
    description: "The playbook binary hooks for Claude Code: session context, worktree creation and removal, and the installer shim.",
    source: {source: "archive", url: $url, sha256: $sha},
    category: "workflow",
    homepage: "https://github.com/pragmatic-engineer/playbook",
    license: "Apache-2.0",
    author: .plugins[0].author
  };
  if any(.plugins[]; .name == "playbook-hooks")
  then (.plugins[] | select(.name == "playbook-hooks") | .source) |= {source: "archive", url: $url, sha256: $sha}
  else .plugins += [entry]
  end
' "$mkt" >"$work/marketplace.json"
cp "$work/marketplace.json" "$mkt"
