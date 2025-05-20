use super::AppMsg;
use std::path::PathBuf;

pub async fn choose_file_to_open() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .add_filter("nexus", &["tree", "trees", "nex", "nexus"])
        .pick_file()
        .await;
    AppMsg::PathToOpen(chosen.map(|pb| pb.path().into()))
}

pub async fn choose_file_to_save() -> AppMsg {
    let chosen =
        rfd::AsyncFileDialog::new().add_filter("newick", &["newick", "tre"]).save_file().await;
    AppMsg::PathToSave(chosen.map(|pb| pb.path().into()))
}

pub fn read_text_file(path_buf: PathBuf) -> String {
    let data = std::fs::read(path_buf)
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .unwrap();
    String::from_utf8(data).unwrap()
}

#[allow(dead_code)]
pub fn write_text_file(path_buf: &PathBuf, s: &str) {
    std::fs::write(path_buf, s)
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .unwrap();
}
