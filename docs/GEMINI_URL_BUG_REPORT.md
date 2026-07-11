# Bug Report: `RelativeUrlWithoutBase` Panic in Gemini API Handler

**File affected:** `src/ai/handlers.rs`
**Severity:** Critical — server panics on every AI copilot request
**Status:** Fixed

---

## The Panic

```
thread 'tokio-runtime-worker' panicked at 'CRITICAL: The string is broken beyond repair!':
called `Result::unwrap()` on an `Err` value: RelativeUrlWithoutBase
```

---

## Root Cause

The URL string on line 59 was formatted as a **Markdown hyperlink** — almost certainly
copy-pasted from a rendered README or web page — instead of a plain string literal.

### What was in the source code

```rust
let raw_url = "[https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent](https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent)";
```

Visually, this is the Markdown syntax `[link text](url)`. The actual string value at
runtime was:

```
[https://generativelanguage.googleapis.com/...](https://generativelanguage.googleapis.com/...)
^
└── starts with '[', not a valid URL scheme character
```

### Why `url::Url::parse` fails

The `url` crate (used internally by `reqwest`) identifies a URL scheme by scanning for the
pattern `scheme://` at the start of the string, where `scheme` must match
`[a-zA-Z][a-zA-Z0-9+\-.]*`.

When the parser encounters `[` as the very first character, it cannot find a valid scheme.
It then falls back to treating the entire string as a **relative URL reference**. Because
`Url::parse` is called with no base URL, the crate returns:

```
Err(RelativeUrlWithoutBase)
```

The `.expect()` call then panics with the message above.

### Why the `is_ascii_graphic()` filter did not help

The code attempted to strip invisible/corrupted characters before parsing:

```rust
let clean_url: String = raw_url.chars().filter(|c| c.is_ascii_graphic()).collect();
```

`is_ascii_graphic()` returns `true` for all printable, non-whitespace ASCII characters
(codepoints `0x21`–`0x7E`). The offending characters `[`, `]`, `(`, `)` are all in that
range, so they passed through the filter completely untouched.

---

## Secondary Findings

### `.env` — CRLF line endings

```
DATABASE_URL=postgresql://...^M$
RUST_LOG=info^M$
GEMINI_API_KEY=AQ.Ab8RN...^M$
```

Every line has a Windows `CRLF` (`\r\n`) ending. This is **benign** for two reasons:

1. `dotenvy` (the `.env` loader used in this project) automatically strips `\r` on Windows.
2. The existing `is_ascii_graphic()` filter on the raw key value provides a second layer of
   defence, since `\r` (`0x0D`) and `\n` (`0x0A`) are both below `0x21` and are filtered out.

### `Cargo.toml` — non-existent `reqwest` feature

```toml
reqwest = { version = "0.13.4", features = ["json", "query"] }
```

`"query"` is not a declared feature in `reqwest`. The `query_pairs_mut()` method used in the
handler is defined on `url::Url` (a transitive dependency) and requires no feature gate.
Cargo emits a warning for unknown features; it was removed to keep the manifest clean.

---

## The Fix

### `src/ai/handlers.rs`

**Before:**

```rust
// 2. The Ultimate URL Parser
let raw_url = "[https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent](https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent)";
let clean_url: String = raw_url.chars().filter(|c| c.is_ascii_graphic()).collect();

// Parse it manually to a strictly typed `reqwest::Url` struct
let mut parsed_url = reqwest::Url::parse(&clean_url)
    .expect("CRITICAL: The string is broken beyond repair!");
```

**After:**

```rust
// 2. Build the Gemini API URL and append the API key as a query parameter.
let mut parsed_url = reqwest::Url::parse(
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent",
)
.expect("Hardcoded Gemini URL must always be valid");
```

The intermediate `raw_url` / `clean_url` variables were redundant for a hardcoded string
literal and are removed entirely.

### `Cargo.toml`

**Before:**

```toml
reqwest = { version = "0.13.4", features = ["json", "query"] }
```

**After:**

```toml
reqwest = { version = "0.13.4", features = ["json"] }
```

---

## How to Avoid This in the Future

| Scenario | Recommendation |
|---|---|
| Copying a URL from a browser, README, or chat | Paste into a plain-text editor first to strip any Markdown/HTML formatting before putting it in source code |
| Hardcoded URLs | Use a bare string literal — no sanitisation loop is needed or helpful |
| Reading URLs from config/env | Validate with `Url::parse` at startup and surface a clear error message rather than panicking mid-request |
| Suspicious `RelativeUrlWithoutBase` error | The input string is missing a scheme (`https://`) — check the very first character for `[`, whitespace, or other non-alpha characters |
