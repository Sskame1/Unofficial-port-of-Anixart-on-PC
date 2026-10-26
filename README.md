# Anixart for Windows

A Rust desktop client for browsing Anixart. The UI is built from local HTML, CSS, and JavaScript modules bundled into a Windows WebView.

## Build

Requirements: Windows x64, Rust, and Microsoft Edge WebView2 Runtime.

```powershell
cargo run --release
```

To select another Anixart API endpoint, set `ANIXART_API_URL` to one of `https://api-s.anixsekai.com/`, `https://api.anixart.app/`, or `https://api.anixart.tv/` before launching.

## ToDo

- Confirm playback of an episode from start to finish.
- Complete catalog, profile, collection, and social feature parity.
- Verify API mirrors and account actions against live service responses.
- Improve Windows packaging and release automation.

## License

Project code is MIT licensed. This does not grant rights to Anixart, the supplied APK, media, or third-party brand assets. See [LICENSE](LICENSE).
