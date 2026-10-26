use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let index_path = root.join("web/index.html");
    let style_paths = [
        "web/styles/tokens.css",
        "web/styles/shell.css",
        "web/styles/release-card.css",
        "web/styles/release-detail.css",
    ];
    let script_paths = [
        "web/services/ipc.js",
        "web/components/navigation.js",
        "web/components/release-card.js",
        "web/screens/home.js",
        "web/screens/discover.js",
        "web/screens/search.js",
        "web/screens/release-detail.js",
        "web/screens/auth.js",
        "web/screens/bookmarks.js",
        "web/screens/feed.js",
        "web/screens/profile.js",
        "web/screens/settings.js",
        "web/screens/schedule.js",
        "web/screens/filter.js",
        "web/screens/collections.js",
        "web/screens/notifications.js",
        "web/screens/placeholder.js",
        "web/app.js",
    ];

    println!("cargo:rerun-if-changed={}", index_path.display());
    let mut html = fs::read_to_string(&index_path).expect("read web/index.html");

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

    let output =
        PathBuf::from(env::var_os("OUT_DIR").expect("build output directory")).join("index.html");
    fs::write(output, html).expect("write bundled web entry point");
}

fn replace_once(source: &str, marker: &str, replacement: &str) -> String {
    assert_eq!(
        source.matches(marker).count(),
        1,
        "expected one {marker} marker"
    );
    source.replacen(marker, replacement, 1)
}
