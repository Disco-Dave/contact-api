#![allow(non_snake_case)]

mod common;

use std::collections::HashMap;

use common::spawn_app;

struct Form<'f> {
    name: &'f str,
    email: &'f str,
    message: &'f str,
}

#[derive(serde::Deserialize)]
struct ErrorBody {
    name: Option<String>,
    email: Option<String>,
    message: Option<String>,
}

fn construct_params<'f>(form: &Form<'f>) -> [(&'f str, &'f str); 3] {
    [
        ("name", form.name),
        ("email", form.email),
        ("message", form.message),
    ]
}

async fn submit(
    client: &reqwest::Client,
    addr: &str,
    params: &[(&str, &str)],
) -> reqwest::Response {
    client
        .post(&format!("{}/", &addr))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request")
}

#[derive(serde::Deserialize)]
struct Content {
    Headers: HashMap<String, Vec<String>>,
    Body: String,
}

#[derive(serde::Deserialize)]
struct Item {
    Content: Content,
}

#[derive(serde::Deserialize)]
struct SearchResponse {
    total: usize,
    items: Vec<Item>,
}

#[actix_rt::test]
async fn posting_contact_with_valid_data_returns_a_202() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "Shaggy",
        email: "scooby@mystery.van",
        message: "Let's solve some mysteries, dude.",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::NO_CONTENT, response.status());
    assert_eq!(Some(0), response.content_length());

    let mail_response = client
        .get(&format!(
            "http://{}:{}/api/v2/search",
            app.email_settings.mailhog_host, app.email_settings.mailhog_port
        ))
        .query(&[("kind", "from"), ("query", &app.email_settings.from)])
        .send()
        .await
        .expect("Unable to reach mail hog");

    assert_eq!(reqwest::StatusCode::OK, mail_response.status());

    let search_response = mail_response
        .json::<SearchResponse>()
        .await
        .expect("Unable to parse response.");

    assert_eq!(1, search_response.total);

    let content = search_response
        .items
        .into_iter()
        .nth(0)
        .expect("There should of been one email.")
        .Content;

    assert_eq!("Let's solve some mysteries, dude.", &content.Body);

    let from = content
        .Headers
        .get("From")
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap();
    assert_eq!(&app.email_settings.from, from);

    let subject = content
        .Headers
        .get("Subject")
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap();
    assert_eq!("Shaggy (scooby@mystery.van)", subject);

    let to = content
        .Headers
        .get("To")
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap();
    assert_eq!("bob@fake.fake, beth@fake.fake, george@other.fake", to);
}

#[actix_rt::test]
async fn missing_name_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "",
        email: "scooby@mystery.van",
        message: "Let's solve some mysteries, dude.",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(Some(String::from("Name may not be empty.")), errors.name);
    assert_eq!(None, errors.email);
    assert_eq!(None, errors.message);
}

#[actix_rt::test]
async fn all_whitespace_for_name_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "           ",
        email: "scooby@mystery.van",
        message: "Let's solve some mysteries, dude.",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(Some(String::from("Name may not be empty.")), errors.name);
    assert_eq!(None, errors.email);
    assert_eq!(None, errors.message);
}

#[actix_rt::test]
async fn missing_email_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "Shaggy",
        email: "",
        message: "Let's solve some mysteries, dude.",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(None, errors.name);
    assert_eq!(Some(String::from("Email may not be empty.")), errors.email);
    assert_eq!(None, errors.message);
}

#[actix_rt::test]
async fn all_whitespace_for_email_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "Shaggy",
        email: "             ",
        message: "Let's solve some mysteries, dude.",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(None, errors.name);
    assert_eq!(Some(String::from("Email may not be empty.")), errors.email);
    assert_eq!(None, errors.message);
}

#[actix_rt::test]
async fn missing_message_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "Shaggy",
        email: "scooby@mystery.vam",
        message: "",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(None, errors.name);
    assert_eq!(None, errors.email);
    assert_eq!(
        Some(String::from("Message may not be empty.")),
        errors.message
    );
}

#[actix_rt::test]
async fn all_whitespace_for_message_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "Shaggy",
        email: "scooby@mystery.vam",
        message: "      ",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(None, errors.name);
    assert_eq!(None, errors.email);
    assert_eq!(
        Some(String::from("Message may not be empty.")),
        errors.message
    );
}

#[actix_rt::test]
async fn everything_left_empty_returns_a_400() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let form = Form {
        name: "",
        email: "",
        message: "",
    };

    let params = construct_params(&form);

    let response = submit(&client, &app.address, &params).await;

    assert_eq!(reqwest::StatusCode::BAD_REQUEST, response.status());

    let errors = response
        .json::<ErrorBody>()
        .await
        .expect("Unable to read json body.");

    assert_eq!(Some(String::from("Name may not be empty.")), errors.name);
    assert_eq!(Some(String::from("Email may not be empty.")), errors.email);
    assert_eq!(
        Some(String::from("Message may not be empty.")),
        errors.message
    );
}
