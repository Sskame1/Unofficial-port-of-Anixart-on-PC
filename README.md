# Anixart Video Client for Windows

A small Rust desktop client for looking up an Anixart episode and opening its provider page on Windows x64, without an Android emulator.

## Current status

The MVP client is implemented. It requests episode metadata from Anixart's guest API and loads supported Kodik iframe URLs in an embedded WebView2 window. The default IDs are a sample only; enter IDs for an episode available to you.

The provider page was reached during a manual run, but successful video playback has **not** been confirmed. The tested Kodik page did not start playback, and its direct media endpoint rejected a read-only request because dynamic `IP`, `deadline`, and `sign` validation was required. This client does not generate or bypass those checks. Treat the project as an API and embedded-page prototype, not a working video player.

The application currently supports Kodik iframe episodes only. It does not provide a catalog, search, account login, subtitle controls, or a native playback engine. Anixart's guest API returned episode metadata without an account in the tested case; playback availability may depend on the provider and its own requirements.

## Build and run

Requirements: Windows x64, Rust toolchain, and Microsoft Edge WebView2 Runtime. The WebView2 Runtime is commonly preinstalled on Windows 10/11.

From the project root, run:

```powershell
cargo run --release
```

To build without launching:

```powershell
cargo build --release
```

The executable is written to `target\release\anixart-windows-client.exe`. Enter the release, type, source, and episode-position IDs in the window, then choose **Open episode**. The values initially shown are `19053`, `2`, `7`, and `1`.

## Repository contents

- `src/` — Rust desktop shell and guest API client.
- `web/` — HTML interface shown in WebView2.
- `decompile/` — local APK extraction, JADX output, and research notes; ignored by Git.
- `docs/` — local research and design documents; ignored by Git.

The original `anixart_v10-5.apk` is kept locally and ignored by Git. Decompilation results are analysis material, not restored original source code.

## Roadmap

- [x] Inspect the APK and document the observed API and player flow.
- [x] Confirm guest access to episode metadata for the tested case.
- [x] Create a Rust Windows desktop prototype with an embedded WebView2 and episode lookup.
- [ ] Find a permitted, reliable way to play at least one available episode and verify it on Windows x64.
- [ ] Add basic pause, seek, and close actions once a working playback path is established.
- [ ] Document any additional runtime files and third-party notices required by the verified playback solution.

## License and third-party services

Original project code is licensed under the MIT License; see [`LICENSE`](LICENSE). This does not grant rights to the APK, Anixart service, Kodik, videos, or other third-party materials. Their owners' terms apply. The app currently uses Rust crates including `reqwest`, `serde`, `tao`, and `wry`; see `Cargo.toml` and `Cargo.lock` for the dependency list and versions.
