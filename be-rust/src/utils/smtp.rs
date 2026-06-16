#[cfg(not(test))]
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
#[cfg(not(test))]
use tracing::debug;

use crate::errors::AppResult;

pub fn send_otp(
    smtp_email: &str,
    smtp_password: &str,
    to_email: &str,
    otp: u32,
) -> AppResult<bool> {
    #[cfg(test)]
    {
        let _ = (smtp_email, smtp_password, to_email, otp);
        return Ok(true);
    }

    #[cfg(not(test))]
    {
        let email = Message::builder()
            .from(smtp_email.parse()?)
            .to(to_email.parse()?)
            .subject("OTP Verification")
            .body(format!("Your OTP is : {}", otp).to_string())?;

        let creds = Credentials::new(smtp_email.to_string(), smtp_password.to_string());

        let mailer = SmtpTransport::relay("smtp.gmail.com")?
            .credentials(creds)
            .build();

        mailer.send(&email)?;

        debug!("Sended");

        Ok(true)
    }
}
