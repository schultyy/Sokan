#[path="../src/file.rs"]
mod file;

#[test]
fn is_valid_with_path_and_content() {
    let file = file::FileResource {
        path: "/home/john/hello.txt".to_string(),
        content: "Foo".to_string()
    };
    assert_eq!(file.is_valid(), true);
}

#[test]
fn is_invalid_without_path() {
    let file = file::FileResource {
        path: "".to_string(),
        content: "Foo".to_string()
    };
    assert_eq!(file.is_valid(), false);
}

#[test]
fn is_invalid_without_content() {
    let file = file::FileResource {
        path: "/home/john/hello.txt".to_string(),
        content: "".to_string()
    };
    assert_eq!(file.is_valid(), false);
}
