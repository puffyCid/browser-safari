use std::path::PathBuf;

use browser_safari::{downloads::SafariDownloads, history::SafariHistory};

#[test]
#[ignore = "Grabs live users history data"]
fn system_safari_history_test() {
    let results = SafariHistory::get_users_history().unwrap();
    assert!(results.len() >= 1)
}

#[test]
#[ignore = "Grabs live users downloads data"]
fn system_safari_downloads_test() {
    let results = SafariDownloads::get_users_downloads().unwrap();
    assert!(results.len() >= 1)
}

#[test]
#[should_panic(expected = "BadSQL")]
fn test_safari_bad_history_db() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/BadHistory.db");
    let _ = SafariHistory::get_history(&test_location.display().to_string()).unwrap();
}

#[test]
#[should_panic(expected = "Plist")]
fn test_safari_not_plist() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/badfile.txt");
    let _ = SafariDownloads::get_downloads(&test_location.display().to_string()).unwrap();
}

#[test]
#[should_panic(expected = "Plist")]
fn test_safari_bad_plist() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/Bad.plist");
    let _ = SafariDownloads::get_downloads(&test_location.display().to_string()).unwrap();
}
