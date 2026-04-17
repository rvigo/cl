# Release Workflow

## Prerequisites

- A `GH_PAT` secret set in `cl` repo settings (Settings → Secrets and variables → Actions) with a Personal Access Token with `repo` scope — required to dispatch events to `homebrew-cl`
- "Allow GitHub Actions to create and approve pull requests" enabled in `homebrew-cl` repo settings (Settings → Actions → General → Workflow permissions)

## Steps

### 1. Tag the release

Ensure all changes are merged to `main`, then tag and push:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

### 2. Wait for the Release workflow

The `build_and_release.yaml` workflow triggers automatically on the tag. It will:

- Run `fmt`, `clippy`, and `test`
- Download the source tarball and generate a SHA-256 checksum
- Create a GitHub release with auto-generated notes and attach `checksums.txt`
- Dispatch a `new-release` event to `homebrew-cl` with the tag and SHA-256

### 3. Wait for `brew test-bot` on the homebrew PR

The dispatch triggers `update-formula.yaml` in `homebrew-cl`, which opens a PR (e.g. `formula-update-vX.Y.Z`) updating the formula URL and SHA-256.

The `brew test-bot` workflow (`tests.yaml`) runs automatically on the PR across two platforms — **ubuntu-22.04** and **macOS 13** — building and uploading bottle artifacts for each. Both jobs must succeed before proceeding.

### 4. Apply the `pr-pull` label

Once `brew test-bot` succeeds, add the `pr-pull` label to the PR. This triggers `brew pr-pull`, which:

- Downloads the bottle artifacts
- Adds bottle information to the formula
- Merges the PR into `main`
- Deletes the branch

The release is complete once the PR is merged.

### 5. Update the tap and install

```bash
brew update && brew upgrade cl
```

## Troubleshooting

**`Resource not accessible by integration` on dispatch step**
The `GH_PAT` secret is missing or the token lacks `repo` scope. Fix the secret, then manually re-fire the dispatch:

```bash
SHA256=$(curl -fsSL https://github.com/rvigo/cl/archive/refs/tags/vX.Y.Z.tar.gz -o /tmp/src.tar.gz && sha256sum /tmp/src.tar.gz | awk '{print $1}')

gh api repos/rvigo/homebrew-cl/dispatches \
  --method POST \
  --field event_type=new-release \
  --field "client_payload[tag]=vX.Y.Z" \
  --field "client_payload[sha256]=$SHA256"
```

**`brew pr-pull` fails with `No matching check suite found`**
The `pr-pull` label was applied before `brew test-bot` finished. Remove the label, wait for `test-bot` to complete, then re-add it.

**Formula PR was not created after dispatch**
The `update-formula.yaml` workflow deletes any existing `formula-update-vX.Y.Z` branch before creating a new one, so re-dispatching is safe. If the workflow still fails (e.g. due to a permissions issue), you can delete the branch manually and re-dispatch:

```bash
gh api repos/rvigo/homebrew-cl/git/refs/heads/formula-update-vX.Y.Z --method DELETE
```
