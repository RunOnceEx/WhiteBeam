#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use std::{error::Error,
          io::Write,
          path::PathBuf,
          process::Command,
          process::Stdio};

fn db_init() -> Result<(), Box<dyn Error>> {
    let schema_path: &str = &format!("{}/src/installer/common/resources/schema.sql", env!("PWD"));
    let default_data_path: &str = &format!("{}/src/installer/common/resources/default.sql", env!("PWD"));
    db_load(schema_path)?;
    db_load(default_data_path)?;
    Ok(())
}

pub fn db_optionally_init(release: &str) -> Result<(), Box<dyn Error>> {
    let is_test: bool = release == "test";
    let db_path: PathBuf = platform::get_data_file_path("database.sqlite", release);
    // Always reinitialize database for testing
    if is_test && (&db_path).exists() {
        std::fs::remove_file(&db_path)?;
    }
    let run_init: bool = is_test || !((&db_path).exists());
    if run_init {
        // TODO: Log errors
        db_init()?
    }
    Ok(())
}

pub fn db_load(sql_path: &str) -> std::io::Result<()> {
    let bin_target_path: PathBuf = PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    let sql_data: Vec<u8> = std::fs::read(sql_path)?;
    let mut child = Command::new(bin_target_path).args(&["--load", "stdin"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    {
        // Limited borrow of stdin
        let stdin = match child.stdin.as_mut() {
            Some(s) => s,
            None => return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "WhiteBeam: Could not get handle to stdin"))
        };
        stdin.write_all(&sql_data)?;
    }
    // TODO: _output, debugging information follows:
    let output = child.wait_with_output()?;
    print!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
    if output.stderr.len() > 0 {
        eprint!("stderr: {}", std::str::from_utf8(&output.stderr).unwrap());
    }
    Ok(())
}

pub fn load_test_data() -> Result<(), Box<dyn Error>> {
    let test_data_path: &str = &format!("{}/src/installer/platforms/{}/resources/test.sql", env!("PWD"), std::env::consts::OS);
    db_load(test_data_path)?;
    Ok(())
}
