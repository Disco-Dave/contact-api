use std::error::Error;

use lettre::{transport::smtp::client::Tls, AsyncTransport};

use super::domain::contact::Contact;
use super::settings::EmailSettings;

pub struct EmailService {
    smtp: lettre::AsyncSmtpTransport<lettre::Tokio1Executor>,
    file: lettre::AsyncFileTransport<lettre::Tokio1Executor>,
    from: String,
    recipients: Vec<String>,
}

impl EmailService {
    pub fn new(settings: EmailSettings) -> Self {
        let smtp = lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&settings.smtp_host)
            .expect("Unable to connect to smtp relay.")
            .port(settings.smtp_port)
            .tls(Tls::None)
            .build();

        std::fs::create_dir_all(&settings.backup_dir).expect("Unable to create backup email dir.");
        let file = lettre::AsyncFileTransport::new(&settings.backup_dir);

        Self {
            smtp,
            file,
            from: settings.from,
            recipients: settings.recipients,
        }
    }

    #[tracing::instrument(name = "Process contact to send email", skip(self))]
    pub async fn send(&self, contact: Contact) -> Result<(), Box<dyn Error>> {
        let builder = lettre::message::Message::builder()
            .from(self.from.parse()?)
            .subject(format!("{} ({})", &contact.name, &contact.email));

        let message = self
            .recipients
            .iter()
            .fold::<Result<lettre::message::MessageBuilder, Box<dyn Error>>, _>(
                Ok(builder),
                |builder, recipient| {
                    builder.and_then(|b| {
                        let mail_box = recipient.to_owned().parse()?;
                        Ok(b.to(mail_box))
                    })
                },
            )?
            .body(contact.message.to_string())?;

        tracing::info!("Message built.");

        self.file.send(message.clone()).await?;
        tracing::info!("Message saved to file system.");

        self.smtp.send(message).await?;
        tracing::info!("Message sent via smtp.");

        Ok(())
    }
}
