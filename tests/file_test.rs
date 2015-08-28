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

#[test]
fn returns_no_error_messages_if_valid() {
    let file = file::FileResource {
        path: "/home/john/hello.txt".to_string(),
        content: "Foobar".to_string()
    };
    assert_eq!(file.error_messages().is_empty(), true);
}

#[test]
fn returns_error_message_if_path_is_missing() {
    let file = file::FileResource {
        path: "".into(),
        content: "FooBar".into()
    };

    let messages = file.error_messages();
    assert_eq!(messages.first(), Some(&"File: path is missing".to_string()));
}

#[test]
fn returns_error_message_if_content_is_missing() {
    let file = file::FileResource {
        path: "/home/john/hello.txt".into(),
        content: "".into()
    };

    let messages = file.error_messages();
    assert_eq!(messages.first(), Some(&"File: content is missing".to_string()));
}

#[test]
fn returns_a_hash_for_content() {
    let file = file::FileResource {
        path: "/home/john/hello.txt".into(),
        content: "foobar".into()
    };

    assert_eq!(file.hash().len() > 0, true);
}
