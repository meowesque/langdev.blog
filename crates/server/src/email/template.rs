use mail_builder::{MessageBuilder, mime::MimePart};

use crate::env;

pub fn totp(to: String, code: String) -> MessageBuilder<'static> {
  MessageBuilder::new()
    .from(("langdev.blog Verification", "noreply@langdev.blog"))
    .to((to.clone(), to))
    .subject("Time-Based One-Time Passcode")
    .text_body(format!("{}/login/totp?={}", env::get().host, code))
}
