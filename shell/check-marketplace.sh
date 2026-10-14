#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Igor Santos
# SPDX-License-Identifier: Apache-2.0
#
# check-marketplace.sh: check the marketplace listing.
# Every plugin source must be a git-subdir source over https, so installs need
# no SSH key. It must set no ref and no version, so the entry follows the
# default branch and the plugin's own version decides when users get an update.
# The path must be relative and the manifest at that path on the default
# branch must carry the same plugin name. Set OFFLINE=1 to skip the lookup.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MKT="$ROOT/.claude-plugin/marketplace.json"
fail=0
bad() { echo "FAIL $*"; fail=1; }
jq empty "$MKT" 2>/dev/null || { echo "FAIL: marketplace.json is not valid JSON"; exit 1; }
if [ "$(jq -r '.name' "$MKT")" = "pragmatic-engineer" ]; then
  echo "PASS name is pragmatic-engineer"
else
  bad "name must be pragmatic-engineer"
fi
n=$(jq '.plugins | length' "$MKT")
[ "$n" -ge 1 ] || bad "the marketplace lists no plugin"
i=0
while [ "$i" -lt "$n" ]; do
  p=".plugins[$i]"
  name=$(jq -r "$p.name" "$MKT")
  i=$((i + 1))
  stype=$(jq -r "$p.source.source // empty" "$MKT")
  url=$(jq -r "$p.source.url // empty" "$MKT")
  path=$(jq -r "$p.source.path // empty" "$MKT")
  if [ "$stype" != "git-subdir" ]; then
    bad "$name source must be git-subdir (got ${stype:-none})"; continue
  fi
  if ! [[ "$url" =~ ^https://github\.com/([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+)\.git$ ]]; then
    bad "$name url must be an https github .git url (got $url)"; continue
  fi
  repo="${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
  if ! [[ "$path" =~ ^[A-Za-z0-9_.-]+(/[A-Za-z0-9_.-]+)*$ ]] || [[ "/$path/" == *"/../"* ]]; then
    bad "$name path must be a relative path without .. (got ${path:-none})"; continue
  fi
  if [ "$(jq "$p.source | has(\"ref\") or has(\"sha\")" "$MKT")" = true ] ||
     [ "$(jq "$p | has(\"version\")" "$MKT")" = true ]; then
    bad "$name must not set ref, sha or version"; continue
  fi
  echo "PASS $name source is git-subdir over https: $repo $path"
  [ "${OFFLINE:-0}" = 1 ] && continue
  found=$(gh api -H "Accept: application/vnd.github.raw" \
    "repos/$repo/contents/$path/.claude-plugin/plugin.json" 2>/dev/null | jq -r '.name // empty' 2>/dev/null)
  if [ "$found" = "$name" ]; then
    echo "PASS $name resolves: $repo/$path has a manifest with this name"
  else
    bad "$name does not resolve: $repo/$path/.claude-plugin/plugin.json has name '${found:-none}'"
  fi
done
exit "$fail"
