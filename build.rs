use base64::Engine;
use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let index_path = root.join("web/index.html");
    let filter_options_path = root.join("web/data/filter-options.json");
    let style_paths = ["web/styles/desktop.css"];
    let script_paths = [
        "web/services/ipc.js",
        "web/components/navigation.js",
        "web/components/release-card.js",
        "web/screens/home.js",
        "web/screens/discover.js",
        "web/screens/search.js",
        "web/screens/release-detail.js",
        "web/screens/bookmarks.js",
        "web/screens/feed.js",
        "web/screens/profile.js",
        "web/screens/collections.js",
        "web/screens/schedule.js",
        "web/screens/filter.js",
        "web/screens/notifications.js",
        "web/screens/placeholder.js",
        "web/screens/settings.js",
        "web/screens/auth.js",
        "web/app.js",
    ];

    println!("cargo:rerun-if-changed={}", index_path.display());
    let mut html = fs::read_to_string(&index_path).expect("read web/index.html");
    println!("cargo:rerun-if-changed={}", filter_options_path.display());
    let filter_options = fs::read_to_string(filter_options_path).expect("read filter options");
    html = replace_once(
        &html,
        "<!-- FILTER_OPTIONS -->",
        &format!("<script>window.appFilterOptions={filter_options};</script>"),
    );

    let styles = style_paths
        .iter()
        .map(|path| {
            println!("cargo:rerun-if-changed={path}");
            fs::read_to_string(root.join(path))
                .unwrap_or_else(|error| panic!("read {path}: {error}"))
        })
        .collect::<Vec<_>>()
        .join("\n");
    html = replace_once(
        &html,
        "<!-- INLINE_STYLES -->",
        &format!("<style>\n{styles}\n</style>"),
    );

    let scripts = script_paths
        .iter()
        .map(|path| {
            println!("cargo:rerun-if-changed={path}");
            fs::read_to_string(root.join(path))
                .unwrap_or_else(|error| panic!("read {path}: {error}"))
        })
        .collect::<Vec<_>>()
        .join("\n");
    html = replace_once(
        &html,
        "<!-- INLINE_SCRIPTS -->",
        &format!("<script>\n{scripts}\n</script>"),
    );

    let icon = [
        ("web/assets/logo.png", "image/png"),
        ("public/icon.png", "image/png"),
        ("public/icon", "image/png"),
        ("public/icon.ico", "image/x-icon"),
        ("public/icon.svg", "image/svg+xml"),
        ("web/assets/icon.png", "image/png"),
        ("web/assets/icon.svg", "image/svg+xml"),
    ]
    .iter()
    .find_map(|(path, mime)| {
        println!("cargo:rerun-if-changed={path}");
        let source = root.join(path);
        let mut bytes = fs::read(&source).ok()?;
        // Wry's Windows `with_html` is limited to 2 MiB. Embedding the full
        // 1254x1254 source logo as base64 made the generated page exceed that
        // limit and caused WebView2 to fail with E_INVALIDARG. Keep the source
        // asset intact and downsample only the build output used in the sidebar.
        if *path == "web/assets/logo.png" && bytes.len() > 128 * 1024 {
            let scaled = PathBuf::from(env::var_os("OUT_DIR").expect("build output directory"))
                .join("sidebar-logo.png");
            resize_png_for_sidebar(&source, &scaled);
            bytes = fs::read(scaled).expect("read resized sidebar logo");
        }
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        Some(format!(
            "<img src=\"data:{mime};base64,{encoded}\" alt=\"\" />"
        ))
    })
    .unwrap_or_default();
    html = replace_once(&html, "<!-- APP_ICON -->", &icon);

    let output =
        PathBuf::from(env::var_os("OUT_DIR").expect("build output directory")).join("index.html");
    fs::write(output, html).expect("write bundled web entry point");
}

fn resize_png_for_sidebar(source: &std::path::Path, target: &std::path::Path) {
    let script = r#"
Add-Type -AssemblyName System.Drawing
$image = [System.Drawing.Image]::FromFile($env:ANIXART_SIDEBAR_LOGO_SOURCE)
try {
  $bitmap = New-Object System.Drawing.Bitmap 128, 128, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
  try {
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    try {
      $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
      $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
      $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
      $graphics.DrawImage($image, 0, 0, 128, 128)
      $bitmap.Save($env:ANIXART_SIDEBAR_LOGO_TARGET, [System.Drawing.Imaging.ImageFormat]::Png)
    } finally { $graphics.Dispose() }
  } finally { $bitmap.Dispose() }
} finally { $image.Dispose() }
"#
        .trim();
    let status = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .env("ANIXART_SIDEBAR_LOGO_SOURCE", source)
        .env("ANIXART_SIDEBAR_LOGO_TARGET", target)
        .status()
        .expect("run PowerShell to resize the sidebar logo");
    assert!(
        status.success(),
        "could not resize the sidebar logo for WebView2"
    );
}

fn replace_once(source: &str, marker: &str, replacement: &str) -> String {
    assert_eq!(
        source.matches(marker).count(),
        1,
        "expected one {marker} marker"
    );
    source.replacen(marker, replacement, 1)
}
