use std::path::Path;
use tiny68_assets::security::validate_pack_path;

#[test]
fn test_path_traversal_detection() {
    let base = Path::new("C:\\Tiny68\\packs");
    let malicious = Path::new("..\\..\\Windows\\System32\\cmd.exe");

    assert!(
        validate_pack_path(base, malicious).is_err(),
        "Path traversal with .. should be rejected"
    );
}

#[test]
fn test_valid_pack_relative_path() {
    let base = Path::new("C:\\Tiny68\\packs");
    let valid = Path::new("mochi\\mochi_atlas.png");

    assert!(validate_pack_path(base, valid).is_ok());
}

