use csv;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use std::{env, error::Error, fs::OpenOptions, io::Write};

use browser_safari::{downloads::SafariDownloads, history::SafariHistory};
fn main() {
    println!("Getting Safari data...");
    SimpleLogger::init(LevelFilter::Warn, Config::default())
        .expect("Failed to initialize simple logger");

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let path = &args[1];
        if path.ends_with(".db") {
            let history_results = SafariHistory::get_history(path);
            match history_results {
                Ok(results) => {
                    let history = vec![SafariHistory {
                        results,
                        path: String::new(),
                        user: String::new(),
                    }];
                    output_history(&history).unwrap();
                }
                Err(err) => println!("Failed to get history data: {:?}", err),
            }
        } else if path.ends_with(".plist") {
            let download_reults = SafariDownloads::get_downloads(path);
            match download_reults {
                Ok(results) => {
                    let downlaods = vec![SafariDownloads {
                        results,
                        path: String::new(),
                        user: String::new(),
                    }];
                    output_downloads(&downlaods).unwrap();
                }
                Err(err) => println!("Failed to get downloads data: {:?}", err.to_string()),
            }
        }
    } else {
        let history_results = SafariHistory::get_users_history();
        match history_results {
            Ok(results) => output_history(&results).unwrap(),
            Err(err) => println!("Failed to get history data: {:?}", err),
        }

        let download_reults = SafariDownloads::get_users_downloads();
        match download_reults {
            Ok(results) => output_downloads(&results).unwrap(),
            Err(err) => println!("Failed to get downloads data: {:?}", err),
        }
    }
}

fn output_history(results: &[SafariHistory]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("output_history.csv")?;
    let mut json_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("output_history.json")?;

    writer.write_record(&[
        "ID",
        "URL",
        "Domain Expansion",
        "Visit Count",
        "Visit Count Score",
        "Status Code",
        "Visit Time",
        "Load Successful",
        "Title",
        "Attributes",
        "Score",
        "User",
        "Path",
    ])?;
    for result in results {
        for history in &result.results {
            writer
                .write_record(&[
                    history.id.to_string(),
                    history.url.to_string(),
                    history.title.as_ref().unwrap_or(&String::new()).to_string(),
                    history.visit_count.to_string(),
                    history.visit_count_score.to_string(),
                    history.status_code.to_string(),
                    history.visit_time.to_string(),
                    history.load_successful.to_string(),
                    history.title.as_ref().unwrap_or(&String::new()).to_string(),
                    history.attributes.to_string(),
                    history.score.to_string(),
                    result.user.to_string(),
                    result.path.to_string(),
                ])
                .unwrap();
        }
    }
    writer.flush()?;

    let serde_data = serde_json::to_string(&results)?;
    json_file.write_all(serde_data.as_bytes())?;
    println!("\nFinished parsing Safari History data. Saved results to: output_history.csv and output_history.json");
    Ok(())
}

fn output_downloads(results: &[SafariDownloads]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("output_downloads.csv")?;
    let mut json_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("output_downloads.json")?;

    writer.write_record(&[
        "Source URL",
        "Download Path",
        "Sandbox ID",
        "Download Bytes",
        "Download ID",
        "Download Finish",
        "Path to File",
        "CNID Path",
        "File Creation",
        "Volume Path",
        "Volume URL",
        "Volumen UUID",
        "Volume Name",
        "Volume Size",
        "Volume Creation",
        "Volume Flag",
        "Volume Root",
        "Username",
        "UID",
        "Folder Index",
        "Creation Options",
        "User",
        "Path",
    ])?;
    for result in results {
        for downloads in &result.results {
            writer
                .write_record(&[
                    downloads.source_url.to_string(),
                    downloads.download_path.to_string(),
                    downloads.sandbox_id.to_string(),
                    downloads.download_bytes.to_string(),
                    downloads.download_id.to_string(),
                    downloads.download_entry_finish.to_string(),
                    downloads.path.join("/").to_string(),
                    format!("{:?}", downloads.cnid_path),
                    downloads.creation.to_string(),
                    downloads.volume_path.to_string(),
                    downloads.volume_url.to_string(),
                    downloads.volume_uuid.to_string(),
                    downloads.volume_name.to_string(),
                    downloads.volume_size.to_string(),
                    downloads.volume_creation.to_string(),
                    format!("{:?}", downloads.volume_flag),
                    downloads.volume_root.to_string(),
                    downloads.username.to_string(),
                    downloads.uid.to_string(),
                    downloads.folder_index.to_string(),
                    downloads.creation_options.to_string(),
                    result.user.to_string(),
                    result.path.to_string(),
                ])
                .unwrap();
        }
    }
    writer.flush()?;

    let serde_data = serde_json::to_string(&results)?;
    json_file.write_all(serde_data.as_bytes())?;
    println!("\nFinished parsing Safari Downloads data. Saved results to: output_downloads.csv and output_downloads.json");
    Ok(())
}
