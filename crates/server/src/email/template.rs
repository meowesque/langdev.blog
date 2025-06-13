use mail_builder::{MessageBuilder, mime::MimePart};

use crate::env;

pub fn totp() -> MessageBuilder<'static> {
  MessageBuilder::new()
    .from(("bob", "bob@g.com"))
    .to(("bob2", "bob@k.com"))
    .subject("Time-Based One-Time Passcode")
    .text_body(format!("{}/login/totp?={}", env::get().host, "meow"))
}
