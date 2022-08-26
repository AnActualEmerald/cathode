use crate::archive::Archive;

#[test]
fn add_file_to_archive() {
    let mut archive = Archive::new();
    let res = archive.add_file("hello", b"world");

    assert!(res.is_ok());
}

#[test]
fn get_file_from_archive() {
    let mut archive = Archive::new();
    archive.add_file("hello", b"world").unwrap();
    let file = archive.get_file("hello");

    assert!(file.is_ok());
    assert_eq!(
        String::from("world"),
        String::from_utf8(file.unwrap()).unwrap()
    );
}
