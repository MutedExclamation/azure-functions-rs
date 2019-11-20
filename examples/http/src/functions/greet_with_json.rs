use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
    http::Status,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_value};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[func]
pub fn greet_with_json(req: HttpRequest) -> HttpResponse {
    if let Ok(request) = from_slice::<Request>(req.body.as_bytes()) {
        let response = Response {
            message: format!("Hello from Rust, {}!", request.name),
        };
        return to_value(response).unwrap().into();
    }

    HttpResponse::build()
        .status(Status::BadRequest)
        .body("Invalid JSON request.")
        .finish()
}
