use std::path::PathBuf;

use thiserror::Error;

use super::AppMsg;

#[derive(Debug, Error)]
pub enum FileReadError {
    #[error("Could not read file.")]
    InputError,
    #[error("Could not parse file data as string.")]
    CouldNotParseDataAsString,
}

pub async fn choose_file_to_open() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        // .add_filter("newick", &["newick", "tre"])
        // .add_filter("nexus", &["tree", "trees", "nex", "nexus", "t"])
        .pick_file()
        .await;
    AppMsg::PathToOpen(chosen.map(|file_handle| file_handle.path().into()))
}

pub fn choose_file_to_open_sync() -> AppMsg {
    let chosen = rfd::FileDialog::new()
        // .add_filter("newick", &["newick", "tre"])
        // .add_filter("nexus", &["tree", "trees", "nex", "nexus", "t"])
        .pick_file();
    AppMsg::PathToOpen(chosen.map(|path_buf| path_buf.as_path().into()))
}

pub async fn choose_file_to_save() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .save_file()
        .await;
    AppMsg::PathToSave(chosen.map(|file_handle| file_handle.path().into()))
}

pub async fn choose_file_to_pdf_export() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("pdf", &["pdf"])
        .save_file()
        .await;
    AppMsg::PathToSave(chosen.map(|file_handle| file_handle.path().into()))
}

pub fn read_text_file(path_buf: PathBuf) -> Result<String, FileReadError> {
    let result_io = std::fs::read(path_buf);
    if let Ok(data) = result_io {
        let result_parse = String::from_utf8(data);
        if let Ok(s) = result_parse {
            Ok(s)
        } else {
            Err(FileReadError::CouldNotParseDataAsString)
        }
    } else {
        Err(FileReadError::InputError)
    }
}

pub fn write_text_file(path_buf: &PathBuf, s: &str) {
    std::fs::write(path_buf, s)
        .map_err(|e| {
            eprintln!("IO error: {e:?}");
        })
        .unwrap();
}
