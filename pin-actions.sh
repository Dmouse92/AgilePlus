#!/bin/bash
# Pin all GitHub Actions to specific commit SHAs
set -euo pipefail

COMMIT_MSG="ci: pin all GitHub Actions SHA [org-bootstrap-2026-05-03]"

# Determine repos to process
if [ $# -gt 0 ]; then
  REPOS=("$@")
else
  REPOS=(
    heliosApp phenoResearchEngine forgecode helios-cli phenoXdd
    chatta Tasken phenotype-infra HeliosLab argis-extensions
    Civis PhenoObservability KDesktopVirt eyetracker vibeproxy
  )
fi

get_sha() {
  local owner_repo="$1"
  local tag="$2"
  local sha=""
  sha=$(gh api "repos/${owner_repo}/commits/${tag}" --jq '.sha' 2>/dev/null) || true
  if [ -n "$sha" ] && [ "$sha" != "null" ]; then
    echo "$sha"; return 0
  fi
  sha=$(gh api "repos/${owner_repo}/git/refs/tags/${tag}" --jq '.object.sha' 2>/dev/null) || true
  if [ -n "$sha" ] && [ "$sha" != "null" ]; then
    echo "$sha"; return 0
  fi
  return 1
}

pin_repo() {
  local repo="$1"
  local repo_dir="/Users/kooshapari/CodeProjects/Phenotype/repos/${repo}"
  local wf_dir="${repo_dir}/.github/workflows"
  echo ""; echo "=== ${repo} ==="
  if [ ! -d "$wf_dir" ]; then
    echo "  SKIP: no .github/workflows"; return 0
  fi
  cd "$repo_dir"
  local current_branch; current_branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null) || true
  echo "  Branch: ${current_branch}"
  local changed=0
  while IFS= read -r -d '' wf; do
    local wf_name; wf_name=$(basename "$wf")
    local reps_file; reps_file=$(mktemp)
    grep -n 'uses:' "$wf" | grep -v '^\s*#' | while IFS= read -r line; do
      local lineno; lineno=$(echo "$line" | cut -d: -f1)
      local uses_content; uses_content=$(echo "$line" | sed 's/^[0-9]*:[[:space:]]*//')
      if echo "$uses_content" | grep -qE 'uses:\s+([^@[:space:]]+)@([^[:space:]]+)'; then
        local action_ref; action_ref=$(echo "$uses_content" | sed -E 's/.*uses:\s+([^@[:space:]]+)@([^[:space:]]+).*/\1/')
        local tag; tag=$(echo "$uses_content" | sed -E 's/.*uses:\s+([^@[:space:]]+)@([^[:space:]]+).*/\2/')
        if echo "$tag" | grep -qE '^[0-9a-f]{40}$'; then continue; fi
        if echo "$action_ref" | grep -q '^\.'; then continue; fi
        local api_repo
        if echo "$action_ref" | grep -qE '^[^/]+/[^/]+/'; then
          api_repo=$(echo "$action_ref" | cut -d/ -f1-2)
        else
          api_repo="$action_ref"
        fi
        local sha; sha=$(get_sha "$api_repo" "$tag" 2>/dev/null) || sha=""
        if [ -n "$sha" ] && [ "$sha" != "null" ]; then
          echo "${lineno}|${action_ref}@${tag}|${sha}" >> "$reps_file"
        else
          echo "  WARN: ${action_ref}@${tag} - could not resolve SHA"
        fi
      fi
    done
    if [ -s "$reps_file" ]; then
      while IFS='|' read -r lineno old_full new_sha; do
        local at_pos; at_pos=$(echo "$old_full" | rev | cut -d@ -f1 | rev)
        local action_path; action_path=$(echo "$old_full" | sed 's/@.*//')
        sed -i '' "${lineno}s|@${at_pos}|@${new_sha}|g" "$wf"
        changed=1
        echo "  PINNED line ${lineno}: ${action_path}@${at_pos} -> ${new_sha}"
      done < "$reps_file"
    fi
    rm -f "$reps_file"
  done < <(find "$wf_dir" -name '*.yml' -print0 -o -name '*.yaml' -print0)
  if [ $changed -eq 1 ]; then
    git add -A
    git commit -m "$COMMIT_MSG" 2>/dev/null || echo "  Nothing to commit"
    git pull --rebase origin "$current_branch" 2>&1 | tail -3 || true
    git push origin "$current_branch" 2>&1 | tail -3 || echo "  Push failed"
    echo "  DONE: ${repo}"
  else
    echo "  No changes for ${repo}"
  fi
}

for repo in "${REPOS[@]}"; do
  pin_repo "$repo" || echo "  ERROR: ${repo}"
done
echo ""; echo "ALL DONE"
