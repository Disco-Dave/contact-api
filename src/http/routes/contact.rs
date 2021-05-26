use std::convert::TryInto;

use crate::{
    domain::contact::{Contact, EmailError, MessageError, NameError},
    email::EmailService,
};
use actix_web::{web::Data, web::Form, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct ContactRequest {
    pub email: String,
    pub name: String,
    pub message: String,
}

#[derive(serde::Serialize, Debug)]
pub struct ContactErrors {
    pub email: Option<&'static str>,
    pub name: Option<&'static str>,
    pub message: Option<&'static str>,
}

impl TryInto<Contact> for ContactRequest {
    type Error = ContactErrors;

    fn try_into(self) -> Result<Contact, Self::Error> {
        Contact::new(&self.email, &self.name, &self.message).map_err(|error| ContactErrors {
            email: error.email.map(|e| match e {
                EmailError::IsEmpty => "Email may not be empty.",
                EmailError::IsMissingAtSign => "Email is missing @ symbol.",
                EmailError::IsGreaterThan300 => "Email may not be longer than 200 characters long.",
            }),
            name: error.name.map(|e| match e {
                NameError::IsEmpty => "Name may not be empty.",
                NameError::IsGreaterThan200 => "Name may not be longer than 200 characters long.",
            }),
            message: error.message.map(|e| match e {
                MessageError::IsEmpty => "Message may not be empty.",
                MessageError::IsGreaterThan2000 => {
                    "Message may not be longer than 2000 characters long."
                }
            }),
        })
    }
}

#[tracing::instrument(name = "Contact handler.", skip(email_service))]
pub async fn handler(
    request: Form<ContactRequest>,
    email_service: Data<EmailService>,
) -> Result<HttpResponse, HttpResponse> {
    tracing::info!("Attempting to parse contact request.");

    let contact = request.0.try_into().map_err(|errors| {
        tracing::info!("Failed to parse contact request: {:?}", errors);
        HttpResponse::BadRequest().json(errors)
    })?;

    tracing::info!("Successfully parsed contact request: {:?}", contact);

    email_service.send(contact).await.map_err(|error| {
        tracing::error!("Failed to process contact: {:?}", error);
        HttpResponse::InternalServerError().finish()
    })?;

    tracing::info!("Successfully processed contact");
    Ok(HttpResponse::NoContent().finish())
}
