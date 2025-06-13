pub mod template;

use crate::{env, prelude::*};
use mail_builder::MessageBuilder;
use mail_send::{SmtpClient, SmtpClientBuilder};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_rustls::client::TlsStream;

const CHANNEL_BUFFER_SIZE: usize = 16;

enum Msg {
  // TODO(m4xine): oneshot for email Response?
  Send(MessageBuilder<'static>),
}

struct Actor {
  rx: mpsc::Receiver<Msg>,
  client: SmtpClient</*TlsStream<*/ TcpStream /*>*/>,
}

impl Actor {
  fn new(rx: mpsc::Receiver<Msg>, client: SmtpClient</*TlsStream<*/ TcpStream /*>*/>) -> Self {
    Self { rx, client }
  }

  async fn run(mut self) {
    while let Some(msg) = self.rx.recv().await {
      self.msg(msg).await;
    }
  }

  async fn msg(&mut self, msg: Msg) {
    match msg {
      Msg::Send(message) => self.send(message).await,
    }
  }

  async fn send(&mut self, message: MessageBuilder<'static>) {
    log::debug!("Sending email: {:?}", message);
    match self.client.send(message).await {
      Ok(response) => {
        log::info!("Sent email, got response: {:?}", response);
      }
      Err(e) => log::warn!("Failed to send email: {}", e),
    }
  }
}

#[derive(Clone)]
pub struct EmailService {
  tx: mpsc::Sender<Msg>,
}

impl EmailService {
  /// Creates an email service with a new connection to an SMTP relay
  /// with its credentials provided via environment variables.
  pub async fn from_env() -> Result<Self> {
    let client = SmtpClientBuilder::new(env::get().smtp_relay.clone(), env::get().smtp_port)
      //.implicit_tls(true)
      //.credentials((env::get().smtp_user.clone(), env::get().smtp_pass.clone()))
      .connect_plain()
      .await?;

    let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
    let actor = Actor::new(rx, client);

    tokio::spawn(async move { actor.run().await });

    Ok(Self { tx })
  }

  pub async fn send(&self, message: MessageBuilder<'static>) -> Result<()> {
    let _ = self.tx.send(Msg::Send(message)).await;
    Ok(())
  }
}
