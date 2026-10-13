#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Igor Santos
# SPDX-License-Identifier: Apache-2.0
#
# check-marketplace.sh: pin the keyless-install invariant. Every plugin source
# MUST be a url or archive source over https so users without SSH keys can
# install. A github source clones via SSH and breaks keyless installs. An
# archive source MUST also carry a 64 hex character sha256. A relative path
# source (a plugin that lives in this repo) is allowed and must exist here.
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
  if [ "$(jq -r ".plugins[$i].source | type" "$MKT")" = "string" ]; then
    rel=$(jq -r ".plugins[$i].source" "$MKT")
    case "$rel" in
      ./*) ;;
      *) echo "FAIL $name relative source must start with ./ (got $rel)"; fail=1; i=$((i+1)); continue ;;
    esac
    if [ -f "$ROOT/$rel/.claude-plugin/plugin.json" ]; then
      echo "PASS $name source is a relative path with a manifest: $rel"
    else
      echo "FAIL $name relative source $rel has no .claude-plugin/plugin.json"; fail=1
    fi
    i=$((i+1)); continue
  fi
  stype=$(jq -r ".plugins[$i].source.source" "$MKT")
  surl=$(jq -r ".plugins[$i].source.url" "$MKT")
  ssha=$(jq -r ".plugins[$i].source.sha256 // empty" "$MKT")
  if ! printf '%s' "$surl" | grep -q '^https://'; then
    echo "FAIL $name source must be https (got $stype / $surl)"; fail=1
  elif [ "$stype" = "url" ]; then
    echo "PASS $name source is url+https (keyless): $surl"
  elif [ "$stype" = "archive" ] && printf '%s' "$ssha" | grep -Eq '^[0-9a-f]{64}$'; then
    echo "PASS $name source is archive+https+sha256 (keyless): $surl"
  else
    echo "FAIL $name source must be url, or archive with a sha256 (got $stype / $surl / ${ssha:-no sha256})"; fail=1
  fi
  i=$((i+1))
done
exit "$fail"
