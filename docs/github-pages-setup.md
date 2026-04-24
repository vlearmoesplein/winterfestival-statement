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

## 6) Configure Custom Domain (`vlearmoesplein.nl`)

To use `vlearmoesplein.nl` for this Pages site:

1. Open **Settings** > **Pages** in the repository.
2. In **Custom domain**, set: `vlearmoesplein.nl`.
3. In your DNS provider:
   - for apex/root domain (`vlearmoesplein.nl`), create `A` records:
     - `185.199.108.153`
     - `185.199.109.153`
     - `185.199.110.153`
     - `185.199.111.153`
   - optionally add `AAAA` records for IPv6:
     - `2606:50c0:8000::153`
     - `2606:50c0:8001::153`
     - `2606:50c0:8002::153`
     - `2606:50c0:8003::153`
   - for `www.vlearmoesplein.nl`, create a `CNAME` to `<your-username>.github.io` (without repository name)
4. Wait for DNS propagation, then verify in **Settings** > **Pages**.
5. Enable **Enforce HTTPS** once the certificate is issued.

Recommended hardening:

- Verify the custom domain in GitHub to reduce domain takeover risk.
- Do not use wildcard DNS records (for example `*.vlearmoesplein.nl`) to avoid takeover risk.

Verification examples:

```bash
dig vlearmoesplein.nl +noall +answer -t A
dig vlearmoesplein.nl +noall +answer -t AAAA
dig www.vlearmoesplein.nl +nostats +nocomments +nocmd
```

Notes:

- For GitHub Pages published from **GitHub Actions**, GitHub does not create/use a repository `CNAME` file.
- GitHub recommends adding the custom domain in repository settings before DNS changes.

## Notes for This Project

- The workflow computes Trunk `--public-url` automatically:
  - `"/"` for `username.github.io` repositories
  - `"/<repo-name>/"` for project repositories
- You can override this by setting repository variable `PAGES_PUBLIC_URL` (for custom domains, use `/`).
- The workflow also downloads Tailwind standalone and daisyUI plugin files before `trunk build`.
- Quality checks run before deploy:
  - `make check`
  - `make clippy`
  - `make test`

## Troubleshooting

- **404 or missing assets**: confirm Pages source is **GitHub Actions** and deploy succeeded.
- **Workflow fails on build**: inspect the failed job logs in **Actions**.
- **Wrong base path**: verify repository naming and `Compute Public URL` step output.
- **Blank page with empty DOM on custom domain**: set repository variable `PAGES_PUBLIC_URL` to `/`, then redeploy.

## References

- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Configuring a custom domain for your GitHub Pages site](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
