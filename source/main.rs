//! a simple calculator with simple calculating abilities, built with actix-web
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
// This struct is used to deserialize query parameters from the form submission
struct CalculationQuery {
    a: i128,
    b: i128,
    operation: String,
}

// this function is the entry point of the application, it sets up the HTTP server and routes
fn main() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async {
        let server = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(create_framework))
                .route("/calculate", web::get().to(calculate_from_form))
                .route("/add/{a}/{b}", web::get().to(add_handler))
                .route("/subtract/{a}/{b}", web::get().to(subtract_handler))
                .route("/multiply/{a}/{b}", web::get().to(multiply_handler))
                .route("/divide/{a}/{b}", web::get().to(divide_handler))
        })
        .bind(("127.0.0.1", 8705))?;

        println!("Rustulator is running at http://127.0.0.1:8705");
        server.run().await
    })
}

// an asynchronous function that serves the HTML framework for the calculator
async fn create_framework() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../html_stuff/index.html"))
}

// another asynchronous function that handles the calculation request from the form submission
async fn calculate_from_form(query: web::Query<CalculationQuery>) -> HttpResponse {
    let CalculationQuery { a, b, operation } = query.into_inner();
    let result = calculate(&operation, a, b);
    let (heading, message) = match result {
        Ok(value) => ("Result", value.to_string()),
        Err(error) => ("Calculation error", error.to_owned()),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        // most of this is dead code at the moment, since we use the index.html file for the main page, but this is a fallback in case the user navigates to /calculate directly
        .body(format!(
            r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Rustulator result</title>
</head>
<body>
    <main>
        <h1>{heading}</h1>
        <p>{message}</p>
        <a href="/">Back to Rustulator</a>
    </main>
</body>
</html>"#
        ))
}

// a handler for adding
async fn add_handler(path: web::Path<(i128, i128)>) -> HttpResponse {
    let (a, b) = path.into_inner();
    api_calculation_response(calculate("add", a, b))
}
// a handler for subtracting
async fn subtract_handler(path: web::Path<(i128, i128)>) -> HttpResponse {
    let (a, b) = path.into_inner();
    api_calculation_response(calculate("subtract", a, b))
}

// a handler for multiplying
async fn multiply_handler(path: web::Path<(i128, i128)>) -> HttpResponse {
    let (a, b) = path.into_inner();
    api_calculation_response(calculate("multiply", a, b))
}

// a handler for dividing
async fn divide_handler(path: web::Path<(i128, i128)>) -> HttpResponse {
    let (a, b) = path.into_inner();
    api_calculation_response(calculate("divide", a, b))
}

// the helper function that acts as the formatter for the API response(s), it takes a Result<i128, &'static str> and returns an HttpResponse
fn api_calculation_response(result: Result<i128, &'static str>) -> HttpResponse {
    match result {
        Ok(value) => HttpResponse::Ok().body(value.to_string()),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

// the calculation logic, it takes an operation as a string and two i128 values, and returns a Result<i128, &'static str> indicating success or failure
fn calculate(operation: &str, a: i128, b: i128) -> Result<i128, &'static str> {
    match operation {
        "add" => a.checked_add(b).ok_or("Result is outside the i128 range"),
        "subtract" => a.checked_sub(b).ok_or("Result is outside the i128 range"),
        "multiply" => a.checked_mul(b).ok_or("Result is outside the i128 range"),
        "divide" => {
            if b == 0 {
                Err("Cannot divide by zero")
            } else {
                a.checked_div(b).ok_or("Result is outside the i128 range")
            }
        }
        _ => Err("Unknown operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn calculates_signed_values_beyond_i32() {
        assert_eq!(
            calculate("add", i32::MAX as i128 + 1, 1),
            Ok(i32::MAX as i128 + 2)
        );
        assert_eq!(calculate("add", u64::MAX as i128, 0), Ok(u64::MAX as i128));
        assert_eq!(calculate("subtract", 5, 12), Ok(-7));
    }

    #[test]
    fn detects_i128_overflow() {
        assert_eq!(
            calculate("add", i128::MAX, 1),
            Err("Result is outside the i128 range")
        );
        assert_eq!(
            calculate("divide", i128::MIN, -1),
            Err("Result is outside the i128 range")
        );
    }

    #[test]
    fn preserves_division_and_operation_errors() {
        assert_eq!(calculate("divide", 1, 0), Err("Cannot divide by zero"));
        assert_eq!(calculate("power", 2, 3), Err("Unknown operation"));
    }
}
// well, that's the end (for now) of the code, I hope you enjoyed reading it, and I hope you enjoy using Rustulator!
// Please refer to the readme.md file for more information on how to use Rustulator, and feel free to contribute to the project if you have any ideas or improvements!
