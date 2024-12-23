use derive_static_str::{static_str, DeriveStaticStr};
use strum::EnumString;

#[derive(EnumString, DeriveStaticStr)]
#[static_str(prefix = "assets/icons/", suffix = ".svg")]
#[strum(serialize_all = "snake_case")]
enum Icon {
    Plus,
    Minus,
    UpArrow,
    DownArrow,
}

#[test]
fn test_derive_static_str_icons() {
    assert_eq!(Icon::Plus.as_static_str(), "assets/icons/plus.svg");
    assert_eq!(Icon::Minus.as_static_str(), "assets/icons/minus.svg");
    assert_eq!(Icon::UpArrow.as_static_str(), "assets/icons/up_arrow.svg");
    assert_eq!(
        Icon::DownArrow.as_static_str(),
        "assets/icons/down_arrow.svg"
    );
}

#[derive(EnumString, DeriveStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}

#[test]
fn test_derive_static_str_http_status() {
    assert_eq!(HttpStatus::Ok.as_static_str(), "OK");
    assert_eq!(HttpStatus::NotFound.as_static_str(), "NOT_FOUND");
    assert_eq!(
        HttpStatus::InternalServerError.as_static_str(),
        "INTERNAL_SERVER_ERROR"
    );
}

#[derive(EnumString, DeriveStaticStr)]
#[static_str(prefix = "api/v1/", suffix = "/")]
enum ApiEndpoint {
    Users,
    Products,
    Orders,
}

#[test]
fn test_derive_static_str_api_endpoints() {
    assert_eq!(ApiEndpoint::Users.as_static_str(), "api/v1/Users/");
    assert_eq!(ApiEndpoint::Products.as_static_str(), "api/v1/Products/");
    assert_eq!(ApiEndpoint::Orders.as_static_str(), "api/v1/Orders/");
}
