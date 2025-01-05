use native_dialog::FileDialog;
use std::fs;
use std::io;
use std::path::Path;

//create hard links
fn create_hard_links(src_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        if src_path.is_file() {
            let file_name = src_path.file_name().unwrap();
            let dest_path = dest_dir.join(file_name);
            fs::hard_link(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let src_dir = FileDialog::new()
        .set_location("~")
        .show_open_single_dir()
        .unwrap()
        .expect("No folder selected");

    let dest_dir = FileDialog::new()
        .set_location("~")
        .show_open_single_dir()
        .unwrap()
        .expect("No folder selected");

    create_hard_links(Path::new(&src_dir), Path::new(&dest_dir))?;
    Ok(())
}
