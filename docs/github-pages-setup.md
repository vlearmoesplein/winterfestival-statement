# GitHub Pages Setup (GitHub Actions)

This repo deploys using `.github/workflows/pages.yml`.

## 1) Push Workflow to Default Branch

Make sure `.github/workflows/pages.yml` is committed on `main`.

## 2) Enable Pages in Repository Settings

1. Open **Settings** in your GitHub repository.
2. Go to **Pages**.
3. Under **Build and deployment**:
   - **Source**: select **GitHub Actions**.

## 3) Confirm Required Permissions

The workflow already sets required permissions:

- `contents: read`
- `pages: write`
- `id-token: write`

No extra manual token setup is needed for standard Pages deploy.

## 4) Trigger a Deployment

Deployment starts when:

- you push to `main`, or
- you run the workflow manually via **Actions** > **Deploy To GitHub Pages** > **Run workflow**

## 5) Verify Site URL

After a successful deploy:

- check the workflow output for the Pages URL, or
- open **Settings** > **Pages** to see the published link.

## Notes for This Project

- The workflow computes Trunk `--public-url` automatically:
  - `"/"` for `username.github.io` repositories
  - `"/<repo-name>/"` for project repositories
- The workflow also downloads Tailwind standalone and daisyUI plugin files before `trunk build`.
- Quality checks run before deploy:
  - `make check`
  - `make clippy`
  - `make test`

## Troubleshooting

- **404 or missing assets**: confirm Pages source is **GitHub Actions** and deploy succeeded.
- **Workflow fails on build**: inspect the failed job logs in **Actions**.
- **Wrong base path**: verify repository naming and `Compute Public URL` step output.

## References

- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
