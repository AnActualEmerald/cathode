use std::io::Cursor;

use base64_url as base64;
use image::ImageFormat;
use log::trace;
use tauri::api::dialog::blocking::FileDialogBuilder;

#[tauri::command]
pub(crate) async fn open_image() -> Option<String> {
    let path = FileDialogBuilder::new()
        .add_filter("Images", &["png", "jpg"])
        .pick_file()?;
    if let Ok(b) = image::open(path) {
        let mut buf = Cursor::new(vec![]);
        b.write_to(&mut buf, ImageFormat::Png).unwrap();
        let encoded = base64::encode(buf.get_ref());
        trace!("Encoded: {:?}", encoded);

        Some(format!("data:image/png;base64,{}", encoded))
    } else {
        None
    }
}
