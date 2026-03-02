use std::path::PathBuf;

use thiserror::Error;

use super::AppMsg;

#[derive(Debug, Error, Clone)]
pub enum FileReadError {
    #[error("Could not read file.\n{file_path}")]
    InputError { file_path: PathBuf },

    #[error("Could not parse file data as text.\n{file_path}")]
    CouldNotParseDataAsText { file_path: PathBuf },
}

pub async fn choose_file_to_open() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new().pick_file().await;
    AppMsg::PathToOpen(chosen.map(|file_handle| file_handle.path().into()))
}

pub fn choose_file_to_open_sync() -> AppMsg {
    let chosen = rfd::FileDialog::new().pick_file();
    AppMsg::PathToOpen(chosen.map(|path_buf| path_buf.as_path().into()))
}

pub async fn choose_file_to_save(subtree: bool) -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .save_file()
        .await;
    AppMsg::PathToSave {
        path: chosen.map(|file_handle| file_handle.path().into()),
        subtree,
    }
}

pub async fn choose_file_to_pdf_export() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("pdf", &["pdf"])
        .save_file()
        .await;
    AppMsg::PathToSave {
        path: chosen.map(|file_handle| file_handle.path().into()),
        subtree: false,
    }
}

pub fn read_text_file(path_buf: PathBuf) -> Result<String, FileReadError> {
    let result_io = std::fs::read(&path_buf);
    if let Ok(data) = result_io {
        let result_parse = String::from_utf8(data);
        if let Ok(s) = result_parse {
            Ok(s)
        } else {
            Err(FileReadError::CouldNotParseDataAsText { file_path: path_buf })
        }
    } else {
        Err(FileReadError::InputError { file_path: path_buf })
    }
}

pub fn write_text_file(path_buf: &PathBuf, s: &str) {
    std::fs::write(path_buf, s)
        .map_err(|e| {
            eprintln!("IO error: {e:?}");
        })
        .unwrap();
}
