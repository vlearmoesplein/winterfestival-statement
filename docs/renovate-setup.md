# Renovate App Setup (GitHub)

This project already includes `renovate.json`.  
To activate automated dependency update PRs, enable the Renovate GitHub App for this repository.

## 1) Install/Enable Renovate

1. Open [Renovate GitHub App](https://github.com/apps/renovate).
2. Click **Install** or **Configure**.
3. Choose your account or organization.
4. Select repositories:
   - **Only select repositories** (recommended), then pick this repo.
5. Confirm installation.

## 2) Verify Repository Config

Ensure `renovate.json` exists in the repo root (already present here) and includes your desired rules.

Current config covers:

- recommended base preset
- dependency dashboard
- grouping for Cargo updates
- grouping for GitHub Actions updates

## 3) Trigger First Run

After installation, Renovate usually opens:

- a **Dependency Dashboard** issue
- one or more dependency update PRs

If not, make a small commit (for example README update) and wait for the next scan cycle.

## 4) Optional GitHub Branch Protection

For safe auto-updates, set branch protection on `main`:

- require status checks
- require pull request reviews
- optionally enable auto-merge for low-risk updates

## Troubleshooting

- **No PRs created**: confirm the app is installed for the correct repo and has access.
- **Only dashboard, no PRs**: check if updates are already up to date or blocked by rules.
- **PRs fail CI**: inspect workflow logs, then adjust update grouping/rules in `renovate.json`.

## References

- [Renovate Documentation](https://docs.renovatebot.com/)
- [Renovate GitHub App](https://github.com/apps/renovate)
