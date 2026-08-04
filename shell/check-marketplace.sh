#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Igor Santos
# SPDX-License-Identifier: MIT
#
# check-marketplace.sh: pin the keyless-install invariant. Every plugin source
# MUST be url + https so users without SSH keys can install. A github source
# clones via SSH and breaks keyless installs.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MKT="$ROOT/.claude-plugin/marketplace.json"
fail=0
jq empty "$MKT" 2>/dev/null || { echo "FAIL: marketplace.json is not valid JSON"; exit 1; }
[ "$(jq -r '.name' "$MKT")" = "pragmatic-engineer" ] && echo "PASS name is pragmatic-engineer" || { echo "FAIL name"; fail=1; }
n=$(jq '.plugins | length' "$MKT")
i=0
while [ "$i" -lt "$n" ]; do
  name=$(jq -r ".plugins[$i].name" "$MKT")
  stype=$(jq -r ".plugins[$i].source.source" "$MKT")
  surl=$(jq -r ".plugins[$i].source.url" "$MKT")
  if [ "$stype" = "url" ] && printf '%s' "$surl" | grep -q '^https://'; then
    echo "PASS $name source is url+https (keyless): $surl"
  else
    echo "FAIL $name source must be url+https (got $stype / $surl)"; fail=1
  fi
  i=$((i+1))
done
exit "$fail"
