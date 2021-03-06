use weber::parser::{
    Builder,
    request::{
        parser::RequestParser,
        builder::RequestBuilder,

        Method::*
    }
};

#[test]
fn request_builder_test() {
    let builded = RequestBuilder::new()
        .set_method(GET)
        .set_header("Content-Type", "text/html")
        .set_path("/foo/bar")
        .set_version(1)
        .build();

    println!("{:?}", builded);

    let parsed = RequestParser::parse(&builded);
    let parsed = parsed.unwrap();

    assert_eq!(parsed.method, GET);
    assert_eq!(parsed.path.path, "/foo/bar");
    assert_eq!(parsed.version, 1);
}
