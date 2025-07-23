# GitHub PTA (Personal Access Token) Setup for CI/CD

## Overview
This guide explains how to set up and use a Personal Access Token (PTA) in GitHub Actions workflows for enhanced authentication and permissions.

## Step 1: Create a Personal Access Token

1. Go to GitHub.com → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Give it a descriptive name (e.g., "GitHub Actions PTA")
4. Set expiration (recommend 90 days for security)
5. Select scopes:
   - `repo` (full control of private repositories)
   - `workflow` (if you need to trigger workflows)
   - `write:packages` (if publishing packages)
   - `read:packages` (if accessing private packages)
6. Click "Generate token"
7. **IMPORTANT**: Copy the token immediately - you won't see it again!

## Step 2: Add PTA as Repository Secret

1. Go to your repository: `https://github.com/torbenanderson/learn_rust`
2. Click **Settings** tab
3. In left sidebar: **Secrets and variables** → **Actions**
4. Click **New repository secret**
5. Fill in:
   - **Name**: `GITHUB_TOKEN` (or `PAT_TOKEN`)
   - **Value**: Paste your PTA
6. Click **Add secret**

## Step 3: Use PTA in GitHub Actions Workflows

### Method 1: Using the secret in checkout (for private repos)
```yaml
- name: Checkout code
  uses: actions/checkout@v4
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
```

### Method 2: Using PTA for API calls
```yaml
- name: Call GitHub API
  run: |
    curl -H "Authorization: token ${{ secrets.GITHUB_TOKEN }}" \
         -H "Accept: application/vnd.github.v3+json" \
         https://api.github.com/repos/${{ github.repository }}/issues
```

### Method 3: Using PTA for git operations
```yaml
- name: Git operations
  run: |
    git config --global user.name "GitHub Actions"
    git config --global user.email "actions@github.com"
    git push origin main
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Method 4: Using PTA for package publishing
```yaml
- name: Publish to registry
  run: cargo publish
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Step 4: Security Best Practices

### ✅ Do's:
- Use environment variables: `${{ secrets.GITHUB_TOKEN }}`
- Set appropriate token expiration
- Give minimal required permissions
- Use different tokens for different purposes
- Regularly rotate tokens

### ❌ Don'ts:
- Never commit tokens to code
- Don't use tokens in public workflows without careful review
- Don't share tokens between repositories unnecessarily
- Don't use tokens with excessive permissions

## Step 5: Testing Your Setup

1. Push your changes to trigger the workflow
2. Check the Actions tab in your repository
3. Verify the workflow runs successfully
4. Check logs to ensure PTA is working correctly

## Common Use Cases

### Private Repository Access
```yaml
- name: Checkout private repo
  uses: actions/checkout@v4
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
```

### Cross-Repository Operations
```yaml
- name: Access another repo
  run: |
    curl -H "Authorization: token ${{ secrets.GITHUB_TOKEN }}" \
         https://api.github.com/repos/other-owner/other-repo
```

### Package Publishing
```yaml
- name: Publish to crates.io
  run: cargo publish
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.CRATES_IO_TOKEN }}
```

## Troubleshooting

### Token not working?
1. Check token permissions match your needs
2. Verify token hasn't expired
3. Ensure secret name matches in workflow
4. Check workflow logs for authentication errors

### Permission denied?
1. Verify token has correct scopes
2. Check if repository is private (requires `repo` scope)
3. Ensure token belongs to correct user/organization

## Files in This Repository

- `.github/workflows/rust-ci.yml` - Main CI workflow with PTA integration
- `.github/workflows/example-with-pat.yml` - Example workflow showing PTA usage
- `GITHUB_PTA_SETUP.md` - This guide

## Next Steps

1. Test your workflow by pushing changes
2. Monitor the Actions tab for any issues
3. Consider setting up additional workflows that need PTA
4. Set up token rotation reminders

## Resources

- [GitHub Personal Access Tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [GitHub API Documentation](https://docs.github.com/en/rest) 