#[test]
fn build_script_uses_current_sourceforge_mdk_sdk_names() {
    let build_script = include_str!("../build.rs");

    for expected in [
        "mdk-sdk-windows-clang.7z",
        "mdk-sdk-macOS.tar.xz",
        "mdk-sdk-linux.tar.xz",
        "mdk-sdk-android.7z",
        "mdk-sdk-iOS.tar.xz",
    ] {
        assert!(
            build_script.contains(expected),
            "build.rs should reference {}",
            expected
        );
    }

    assert!(
        !build_script.contains("mdk-sdk-windows-desktop-clang.7z"),
        "build.rs should not reference the obsolete Windows desktop clang package"
    );
}
