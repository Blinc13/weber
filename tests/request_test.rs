use weber::parser::request::builder::{GET, RequestBuilder};
use weber::parser::request::parser::RequestParser;

#[test]
fn request_builder_test() {
    let builded = RequestBuilder::new()
        .set_method(GET)
        .set_header("Content-Type", "text/html")
        .set_path("/foo/bar")
        .set_version(1)
        .build();

    let parsed = RequestParser::parse(&builded);
    let parsed = parsed.unwrap();

    assert_eq!(parsed.method, GET);
    assert_eq!(parsed.path, "/foo/bar");
    assert_eq!(parsed.version, 1);
}