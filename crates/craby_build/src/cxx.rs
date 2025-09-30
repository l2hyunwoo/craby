use std::{fs, path::PathBuf};

use craby_common::config::load_config;

pub fn replace_cxx_header(signal_path: &PathBuf) -> Result<(), anyhow::Error> {
    let signals_h = fs::read_to_string(&signal_path)?;
    let signals_h = signals_h.replace("\"rust/cxx.h\"", "\"cxx.h\"");
    fs::write(&signal_path, signals_h)?;
    Ok(())
}
