use clap::Parser;
use livegrep::{DateRange, LiveGrep};
use std::path::{Path, PathBuf};

mod livegrep;

#[derive(Parser)]
pub struct LiveGrepOpts {
    #[arg(value_name = "DATA_DIR")]
    pub data_dir: PathBuf,

    #[arg(value_name = "REGEX")]
    pub regex: String,

    #[arg(value_name = "date")]
    pub date: String,
}

fn main() {
    let LiveGrepOpts {
        data_dir,
        regex,
        date,
    } = LiveGrepOpts::parse();

    let date_range = match DateRange::parse_date_range(date) {
        Ok(date_range) => date_range,
        Err(error) => {
            println!("Error occurred parsing date range: {error}");
            return;
        }
    };

    let live_grep = LiveGrep { regex, date_range };

    match live_grep.scan_directory(data_dir) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
        }
        Err(error) => {
            println!("Error occurred: {error}");
        }
    }
}

#[test]
fn test_cli() -> anyhow::Result<()> {
    let test_data = Path::new("./NASA_access_log_test");
    let regex = "unicomp[0-9].unicomp.net";
    let date_range = DateRange::parse_date_range("01/Jun/1990-01/Aug/2000".to_string())?;

    let live_grep = LiveGrep {
        regex: regex.to_string(),
        date_range,
    };

    let lines = live_grep.scan_directory(test_data.to_path_buf())?;
    assert_eq!(lines.len(), 4);

    Ok(())
}
