use weber::{
    net::HttpData,
    parser::{
        Builder,
        response::builder::ResponseBuilder
    }
};

#[test]
fn httpdata_test() {
    let resp = ResponseBuilder::new().build();

    let parsed = HttpData::parse(resp.as_bytes()).unwrap();
    let parsed = parsed.as_response();

    assert_eq!(parsed.status_code, 200);
}

#[test]
#[should_panic]
fn httpdata_panic_test() {
    let resp = ResponseBuilder::new().build();

    let parsed = HttpData::parse(resp.as_bytes()).unwrap();
    let _parsed = parsed.as_request();
}