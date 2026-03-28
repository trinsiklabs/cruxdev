# X/Twitter API Setup Report — @splntrb

**Date:** 2026-03-28
**Account:** @splntrb
**App:** 2037864684338946049splntrb (App ID: 32664510)
**Plan:** Pay Per Use ($5.00 credit remaining)

## Completed Steps

### Step 1: Verified App Permissions
- App permissions: **Read and write** (already set correctly)
- Type of App: Web App, Automated App or Bot
- No changes needed — permissions were already correct

### Step 2: Generated All 4 OAuth 1.0a Credentials
Both Consumer Keys and Access Tokens were regenerated fresh on 2026-03-28.

| X Developer Portal Label | Env Var Name | Value |
|---|---|---|
| API Key (Consumer Key) | `X_CLIENT_ID` | `5zrPPR0bImAiftef5Ga37YQU4` |
| API Key Secret (Consumer Key Secret) | `X_CLIENT_SECRET` | `CDbjWYtshEhVgVRc7vrV79Nec4jZi0Yad8oXmwUeAsfD2DFG2A` |
| Access Token | `X_OAUTH_TOKEN` | `1296498068582551557-20eqJUt78qz1wDZei8sj34vQHVARgy` |
| Access Token Secret | `X_OAUTH_TOKEN_SECRET` | `DyZ5003LIMSKxYGPvEwzMF1xnzygv6iSHStog4G06BbjM` |

**Note:** The Access Token was generated with "Read and write" permissions for @splntrb.

### Step 3: Credentials Stored
Added to `~/.zshenv`:

```bash
# X/Twitter API (OAuth 1.0a — for posting tweets)
export X_CLIENT_ID="redacted"
export X_CLIENT_SECRET="redacted"
export X_OAUTH_TOKEN="redacted"
export X_OAUTH_TOKEN_SECRET="redacted"
```

## Remaining Steps

### Step 4: Test Posting
Run this to verify the credentials work end-to-end:

```bash
source ~/.zshenv && python3 << 'PYEOF'
import tweepy, os

client = tweepy.Client(
    consumer_key=os.environ['X_CLIENT_ID'],
    consumer_secret=os.environ['X_CLIENT_SECRET'],
    access_token=os.environ['X_OAUTH_TOKEN'],
    access_token_secret=os.environ['X_OAUTH_TOKEN_SECRET'],
)

resp = client.create_tweet(text="Testing CruxDev X API integration. This is an automated test post.")
print(f"SUCCESS: https://x.com/splntrb/status/{resp.data['id']}")
print("Delete this test tweet from X if needed.")
PYEOF
```

If `tweepy` is not installed: `pip3 install --user --break-system-packages tweepy`

### Troubleshooting

| Error | Fix |
|---|---|
| `401 Unauthorized` | API Key or Secret is wrong. Regenerate Consumer Keys. |
| `403 Forbidden: oauth1 app permissions` | Access Token was generated with Read-only permissions. Regenerate Authentication Tokens. |
| `403 Forbidden: not allowed` | App doesn't have Write permissions. Change in Settings → User authentication settings. |
| `tweepy not found` | `pip3 install --user --break-system-packages tweepy` |

## Additional Notes

- There is also a second app called **cruxdev** (App ID: 32664541) on the same account — not used for this setup.
- The Bearer Token was NOT generated (not needed for OAuth 1.0a tweet posting).
- OAuth 2.0 Client ID is also available (`NtJ0R211d1g5dFVLVXQ2SDhDa0U6MTpjaQ`) but not used for this workflow.
