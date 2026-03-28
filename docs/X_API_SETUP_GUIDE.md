# X/Twitter API Setup Guide — For Claude Cowork

**Goal:** Get working OAuth 1.0a credentials that can POST tweets from the @splntrb account.

**App ID:** 203786464338946049splntrb (already created at developer.x.com)

---

## Step 1: Verify App Permissions

1. Go to https://developer.x.com/en/portal/projects-and-apps
2. Click on the app (203786464338946049splntrb)
3. Go to **Settings** tab (not Keys)
4. Under **User authentication settings** → click **Edit**
5. Verify:
   - **App permissions** = "Read and write" (NOT "Read")
   - **Type of App** = "Web App, Automated App or Bot"
   - **Callback URL** = `https://cruxdev.dev/auth/callback`
   - **Website URL** = `https://cruxdev.dev`
6. Click **Save**

**IMPORTANT:** If you change permissions from "Read" to "Read and write," you MUST regenerate the Access Token and Secret in Step 2. Old tokens keep the old permissions.

---

## Step 2: Get the 4 Required Credentials

Go to the **Keys and Tokens** tab of your app. You need exactly 4 values:

### What X Calls Them → What We Store Them As

| X Developer Portal Label | Our Env Var Name | Where to Find |
|---|---|---|
| **API Key** (under "Consumer Keys") | `X_CLIENT_ID` | Keys and Tokens → Consumer Keys → API Key |
| **API Key Secret** (under "Consumer Keys") | `X_CLIENT_SECRET` | Keys and Tokens → Consumer Keys → API Key Secret |
| **Access Token** (under "Authentication Tokens") | `X_OAUTH_TOKEN` | Keys and Tokens → Authentication Tokens → Access Token |
| **Access Token Secret** (under "Authentication Tokens") | `X_OAUTH_TOKEN_SECRET` | Keys and Tokens → Authentication Tokens → Access Token Secret |

**NOTE:** There's also a "Bearer Token" — we do NOT need it. That's for app-only (read-only) access.

### To Regenerate (required after permission changes):

1. Under **Consumer Keys**: click **Regenerate** → copy both API Key and API Key Secret
2. Under **Authentication Tokens**: click **Regenerate** → copy both Access Token and Access Token Secret
3. **You MUST regenerate Authentication Tokens after changing permissions.** The Access Token/Secret carry the permission level from when they were generated.

---

## Step 3: Store Credentials

Add ALL 4 to `~/.zshenv`:

```bash
# X/Twitter API (OAuth 1.0a — for posting tweets)
export X_CLIENT_ID="your_api_key_here"
export X_CLIENT_SECRET="your_api_key_secret_here"
export X_OAUTH_TOKEN="your_access_token_here"
export X_OAUTH_TOKEN_SECRET="your_access_token_secret_here"
```

Then: `source ~/.zshenv`

---

## Step 4: Test Posting

Run this test (requires `tweepy` Python package):

```bash
source ~/.zshenv && python3 << 'PYEOF'
import tweepy, os

client = tweepy.Client(
    consumer_key=os.environ['X_CLIENT_ID'],
    consumer_secret=os.environ['X_CLIENT_SECRET'],
    access_token=os.environ['X_OAUTH_TOKEN'],
    access_token_secret=os.environ['X_OAUTH_TOKEN_SECRET'],
)

# Test with a simple tweet
resp = client.create_tweet(text="Testing CruxDev X API integration. This is an automated test post.")
print(f"SUCCESS: https://x.com/splntrb/status/{resp.data['id']}")
print("Delete this test tweet from X if needed.")
PYEOF
```

### If it fails:

| Error | Fix |
|---|---|
| `401 Unauthorized` | API Key or Secret is wrong. Regenerate Consumer Keys. |
| `403 Forbidden: oauth1 app permissions` | Access Token was generated with Read-only permissions. Regenerate Authentication Tokens AFTER setting permissions to Read+Write. |
| `403 Forbidden: not allowed` | App doesn't have Write permissions. Go to Settings → User authentication settings → change to "Read and write" → Save → then Regenerate Authentication Tokens. |
| `tweepy not found` | Run: `pip3 install --user --break-system-packages tweepy` |

---

## Step 5: Verify and Report Back

Once the test tweet posts successfully, report back with:
1. Confirmation that the tweet appeared on https://x.com/splntrb
2. The env var names used (should be X_CLIENT_ID, X_CLIENT_SECRET, X_OAUTH_TOKEN, X_OAUTH_TOKEN_SECRET)
3. Delete the test tweet if desired

---

## Common Confusion Points

- **"API Key" vs "Access Token"** — these are DIFFERENT things. API Key identifies your APP. Access Token identifies your USER+APP combination.
- **"Bearer Token"** — this is a THIRD type of credential, for read-only app-level access. We don't use it for posting.
- **Regenerating** — when you regenerate Consumer Keys, the OLD keys stop working immediately. Same for Access Tokens. Update ~/.zshenv right away.
- **Permission propagation** — changing app permissions does NOT update existing tokens. You must regenerate the Access Token and Secret after changing permissions.
