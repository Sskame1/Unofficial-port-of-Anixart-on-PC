<p align="center">
  <img src="web/assets/logo.png" alt="Anixart logo" width="96">
</p>

# Anixart For Windows

## Description

A Rust desktop client for browsing Anixart. The interface uses local HTML, CSS, and JavaScript bundled into a Windows WebView.

## Build

Requirements: Windows x64, Rust, and Microsoft Edge WebView2 Runtime.

```powershell
cargo run --release
```

The application uses the configured Anixart API endpoint and stores its local image cache under `%LOCALAPPDATA%\AnixartWindowsClient\Memory`.

## License

Project code is available under the MIT License. This does not grant rights to Anixart, the supplied APK, media, or third-party brand assets. See [LICENSE](LICENSE).
