use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct MailClient {
    pub mailer: SmtpTransport,
    pub sender_email: String,
}

impl MailClient {
    pub fn new(
        smtp_host: &str,
        smtp_port: u16,
        smtp_user: &str,
        smtp_pass: &str,
        sender_email: &str,
    ) -> Self {
        let creds = Credentials::new(smtp_user.to_string(), smtp_pass.to_string());

        let mailer = SmtpTransport::relay(smtp_host)
            .unwrap()
            .port(smtp_port)
            .credentials(creds)
            .build();

        Self {
            mailer,
            sender_email: sender_email.to_string(),
        }
    }

    pub fn send_mail(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.sender_email.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())?;
        self.mailer.send(&email)?;
        Ok(())
    }
}