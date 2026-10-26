# Anixart for Windows

A Rust desktop port based on the user-provided Anixart APK. The first interface target is the mobile app layout inside a Windows x64 window. This project is being migrated feature by feature; it is not yet a complete replacement for the Android app.

## Current implementation

- The interface follows the APK's mobile layout: top search, five-item bottom bar, horizontal tabs, quick actions, and poster grids.
- Home, release details, search, episode selection, bookmarks, feed, profile, sign-up/sign-in/recovery, settings, notifications, schedule, filters, and collections have separate HTML/CSS screens or navigation transitions. Screens without API integration show explicit account/loading/unsupported states instead of invented data.
- The local `docs/UI_PORT_STATUS.md` inventory tracks APK screen families and remaining port work; `docs/` remains excluded from Git.
- A build-time step bundles the separate HTML, CSS, and JavaScript modules into the local WebView document.
- `cargo run` starts the desktop window on the user's Windows x64 machine. The Home, Discover, Bookmarks, Feed, and Profile sections are present.
- Home and Discover request a random public release through Anixart's guest API. Cards show available fields and open a release details screen.
- Release details load the release, audio/subtitle types, sources, and episodes through the APK-observed API chain. Opening a Kodik iframe hands off to the provider page; video playback is unconfirmed.
- The top search button opens a guest release search screen. APK-observed request details are implemented; end-to-end use after the latest update has not yet been verified.
- Password sign-in follows the APK's form request and response model. Google sign-in follows Google OAuth → Firebase ID token → Anixart `auth/google`; it requires `ANIXART_GOOGLE_CLIENT_ID` to be a desktop OAuth client registered for the same Firebase project as Anixart. The APK only provides an Android client ID, so Google sign-in is not ready for live use until Anixart's desktop OAuth client is available.
- A successful session is persisted in Windows Credential Manager. Authenticated profile, history, favorites, personal lists, and feed requests use the saved token; profile data includes counters, viewing dynamics, and recently watched releases.
- Discover now requests recommendations, discussing, and watching pages; schedule loads all seven weekday lists; the filter submits a year range and APK sort value. These endpoint integrations compile, but this environment's configured proxy prevents live API verification. The filter's remaining APK fields are still to be wired into controls.
- The episode form still requests guest episode metadata and opens the provider's Kodik page in an embedded frame. Actual episode playback has not been confirmed; the provider page may show its own ads.
- The APK has 42 API interfaces and hundreds of layouts; social actions, collections, notifications, and full playback parity are still incomplete.
- A Kodik page was reached, but video playback has not been confirmed. The tested direct media endpoint required dynamic `IP`, `deadline`, and `sign` validation. This project does not bypass that validation.

The user confirmed that `cargo run` previously launched successfully and the catalog returned live data. Several new HTML screens have not been verified in a running window because WebView2 returned `0x8000FFFF`. The current environment cannot reach Anixart because its configured proxy rejects requests at `127.0.0.1:9`; account, schedule, discover, and filter endpoints therefore have compile-time, not live-service, verification. Direct video playback is not confirmed.

## Build and run

Requirements: Windows x64, Rust toolchain, and Microsoft Edge WebView2 Runtime (commonly preinstalled on Windows 10/11).

```powershell
cargo run --release
```

Build without launching:

```powershell
cargo build --release
```

The executable is `target\release\anixart-windows-client.exe`.

## Project layout

- `src/app/` — app state, routes, and command handling.
- `src/api/` — shared HTTP client, configuration, and errors.
- `src/features/player/` — episode lookup and supported provider handoff.
- `web/styles/`, `web/components/`, `web/screens/`, `web/services/` — split local interface modules.
- `decompile/` — local APK extraction and research; ignored by Git.
- `docs/` — local design, migration plans, and inventory; ignored by Git.

## Roadmap

See `docs/superpowers/plans/2026-10-26-anixart-port-roadmap.md` for the staged migration plans: foundation, public discovery, authentication, personal library, social/community, settings/data, and playback/parity.

## License

Original project code is licensed under MIT (see [`LICENSE`](LICENSE)). This does not grant rights to the APK, Anixart, Kodik, video, brand assets, or other third-party materials. Their owners' terms apply.
