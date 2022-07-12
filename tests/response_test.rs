use weber::parser::{
    Builder,
    response::{
        builder::ResponseBuilder,
        parser::ResponseParser
    }
};

#[test]
fn response_builder_test() {
    let reason = "PAGE NOT FOUND";

    let response = ResponseBuilder::new()
        .set_code(200)
        .set_version(1)
        .set_reason(reason)
        .set_header("Test", "test")
        .set_header("Tuc-Tuc", "Xto-tam-?")
        .build(); // Building the response

    let parsed = ResponseParser::parse(&response); // Parsing the response
    let parsed = parsed.unwrap(); // Unwrap the parsed response

    assert_eq!(parsed.status_code, 200);
    assert_eq!(parsed.version, 1);
    assert_eq!(parsed.reason.unwrap(), reason);
}
