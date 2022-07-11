use weber::parser::request::PathParser;

#[test]
#[should_panic]
fn pathparser_panic_test() {
    let path = "foo/bar:val=5:val=6";

    let _parsed = PathParser::parse(&path).unwrap();
}