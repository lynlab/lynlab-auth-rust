extern crate lettre;
extern crate lettre_email;

use std;
use self::lettre::{EmailTransport, SmtpTransport};
use self::lettre::smtp::ConnectionReuseParameters;
use self::lettre::smtp::authentication::{Credentials, Mechanism};
use self::lettre_email::EmailBuilder;


pub fn send_email(address: &str, subject: &str, body: &str) {
    let email = EmailBuilder::new()
        .to(address)
        .from(("no-reply@lynlab.co.kr", "LYnLab"))
        .subject(subject)
        .text(body)
        .build()
        .unwrap();
    
    let sendgrid_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let sendgrid_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let mut sendgrid = SmtpTransport::simple_builder("smtp.sendgrid.net")
        .unwrap()
        .smtp_utf8(true)
        .credentials(Credentials::new(sendgrid_username, sendgrid_password))
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();

    let _ = sendgrid.send(&email);
    sendgrid.close();
}
