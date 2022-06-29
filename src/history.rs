//! Parse Safari History SQLITE file
//!
//! Provides a library to parse Safari data, currently supports:
//!   Safari History
//!   Safari Downloads

use std::{fs::read_dir, path::Path};

use log::{error, info, warn};
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;

use crate::error::SafariError;

#[derive(Debug, Serialize)]
pub struct SafariHistory {
    pub results: Vec<History>,
    pub path: String,
    pub user: String,
}

#[derive(Debug, Serialize)]
pub struct History {
    pub id: i64,
    pub url: String,
    pub domain_expansion: Option<String>, // Domain expansion entry, value is optional (Can be null)
    pub visit_count: i64,
    pub daily_visit_counts: Option<Vec<u8>>,    // Can be null
    pub weekly_visit_counts: Option<Vec<u8>>,   // Can be null
    pub autocomplete_triggers: Option<Vec<u8>>, // Can be null
    pub should_recompute_derived_visit_counts: i64,
    pub visit_count_score: i64,
    pub status_code: i64,
    pub visit_time: f64,
    pub load_successful: bool,
    pub title: Option<String>, // Title entry, value is optional (Can be null)
    pub attributes: f64,
    pub score: f64,
}

impl SafariHistory {
    /// Get Safari SQLITE History file for all users to get browser history
    pub fn get_users_history() -> Result<Vec<SafariHistory>, SafariError> {
        let base_directory = "/Users/";
        let history_path = "/Library/Safari/History.db";
        let users = read_dir(base_directory);

        let mut safari_history: Vec<SafariHistory> = Vec::new();
        match users {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(entry_result) => {
                            let path = format!("{}{}", entry_result.path().display(), history_path);
                            let full_path = Path::new(&path);
                            if !full_path.is_file() {
                                continue;
                            }
                            info!("Parsing file path: {}", path);

                            let results =
                                SafariHistory::get_history(&full_path.display().to_string())?;

                            let username = entry_result
                                .path()
                                .display()
                                .to_string()
                                .replace("/Users/", "");
                            let history = SafariHistory {
                                results,
                                path,
                                user: username,
                            };

                            safari_history.push(history);
                        }
                        Err(err) => warn!("Failed to get user directory: {:?}", err),
                    }
                }
            }
            Err(err) => {
                error!(
                    "Failed to read base directory {}: {:?}",
                    base_directory, err
                );
                return Err(SafariError::PathError);
            }
        }
        Ok(safari_history)
    }

    /// Query the URL history tables based on provided path
    pub fn get_history(path: &str) -> Result<Vec<History>, SafariError> {
        // Bypass SQLITE file lock
        let history_file = format!("file:{}?immutable=1", path);
        let connection = Connection::open_with_flags(
            history_file,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
        );
        let conn = match connection {
            Ok(connect) => connect,
            Err(err) => {
                error!("Failed to read Safari SQLITE history file {:?}", err);
                return Err(SafariError::SQLITEParseError);
            }
        };

        let  statement = conn.prepare("SELECT history_items.id as history_item_id, url, domain_expansion, visit_count, daily_visit_counts,weekly_visit_counts,autocomplete_triggers,should_recompute_derived_visit_counts,visit_count_score,status_code,visit_time,title,load_successful,http_non_get,synthesized,redirect_destination,origin,generation,attributes,score FROM history_items JOIN history_visits ON history_visits.history_item = history_items.id");
        let mut stmt = match statement {
            Ok(query) => query,
            Err(err) => {
                error!("Failed to compose Safari Histoy SQL query {:?}", err);
                return Err(SafariError::BadSQL);
            }
        };

        // Get browser history data
        let history_data = stmt.query_map([], |row| {
            Ok(History {
                id: row.get("history_item_id")?,
                url: row.get("url")?,
                title: row.get("title")?,
                visit_count: row.get("visit_count")?,
                domain_expansion: row.get("domain_expansion")?,
                daily_visit_counts: row.get("daily_visit_counts")?,
                weekly_visit_counts: row.get("weekly_visit_counts")?,
                autocomplete_triggers: row.get("autocomplete_triggers")?,
                should_recompute_derived_visit_counts: row
                    .get("should_recompute_derived_visit_counts")?,
                visit_count_score: row.get("visit_count_score")?,
                status_code: row.get("status_code")?,
                visit_time: row.get("visit_time")?,
                load_successful: row.get("load_successful")?,
                attributes: row.get("attributes")?,
                score: row.get("score")?,
            })
        });

        match history_data {
            Ok(history_iter) => {
                let mut history_vec: Vec<History> = Vec::new();

                for history in history_iter {
                    match history {
                        Ok(history_data) => history_vec.push(history_data),
                        Err(err) => {
                            warn!("Failed to iterate through Safari history data: {:?}", err);
                        }
                    }
                }
                if history_vec.is_empty() {
                    return Err(SafariError::NoHistory);
                }
                Ok(history_vec)
            }
            Err(err) => {
                error!(
                    "Failed to get Safari history data from SQLITE file: {:?}",
                    err
                );
                Err(SafariError::SQLITEParseError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::SafariHistory;

    #[test]
    #[ignore = "Get live users Safari history"]
    fn test_get_users_history() {
        let result = SafariHistory::get_users_history().unwrap();
        assert!(result.len() > 0);
    }

    #[test]
    fn test_safari_history() {
        let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_location.push("tests/test_data/History.db");
        let history = SafariHistory::get_history(&test_location.display().to_string()).unwrap();

        assert_eq!(history.len(), 42);
        assert_eq!(history[0].id, 167);
        assert_eq!(
            history[0].url,
            "https://www.google.com/search?client=safari&rls=en&q=duckduckgo&ie=UTF-8&oe=UTF-8"
        );
        assert_eq!(history[0].domain_expansion.as_ref().unwrap(), "google");
        assert_eq!(history[0].visit_count, 2);
        let daily_visits: Vec<u8> = Vec::from([100, 0, 0, 0]);
        assert_eq!(
            history[0].daily_visit_counts.as_ref().unwrap(),
            &daily_visits
        );
        assert_eq!(history[0].weekly_visit_counts, None);
        assert_eq!(history[0].autocomplete_triggers, None);
        assert_eq!(history[0].should_recompute_derived_visit_counts, 0);
        assert_eq!(history[0].visit_count_score, 100);
        assert_eq!(history[0].status_code, 0);
        assert_eq!(history[0].visit_time, 677386043.546784);
        assert_eq!(history[0].load_successful, true);
        assert_eq!(
            history[0].title.as_ref().unwrap(),
            "duckduckgo - Google Search"
        );
        assert_eq!(history[0].attributes, 0.0);
        assert_eq!(history[0].score, 100.0);

        assert_eq!(history[9].id, 173);
        assert_eq!(
            history[9].url,
            "https://docs.microsoft.com/en-us/powershell/scripting/overview"
        );
        assert_eq!(
            history[9].domain_expansion.as_ref().unwrap(),
            "docs.microsoft"
        );
        assert_eq!(history[9].visit_count, 1);
        let daily_visits: Vec<u8> = Vec::from([100, 0, 0, 0]);
        assert_eq!(
            history[9].daily_visit_counts.as_ref().unwrap(),
            &daily_visits
        );
        assert_eq!(history[9].weekly_visit_counts, None);
        assert_eq!(history[9].autocomplete_triggers, None);
        assert_eq!(history[9].should_recompute_derived_visit_counts, 0);
        assert_eq!(history[9].visit_count_score, 100);
        assert_eq!(history[9].status_code, 0);
        assert_eq!(history[9].visit_time, 677388044.355528);
        assert_eq!(history[9].load_successful, true);
        assert_eq!(history[9].title.as_ref().unwrap(), "");
        assert_eq!(history[9].attributes, 0.0);
        assert_eq!(history[9].score, 100.0);
    }
}
