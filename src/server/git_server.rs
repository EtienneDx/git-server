use std::sync::Arc;

use russh::server::{Config, Server};

use crate::{error::GitError, repository::repository_provider::RepositoryProvider};

use super::{
  authenticator::Authenticator, git_server_config::GitServerConfig, request_handler::RequestHandler,
};

pub struct GitServer<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  authenticator: Arc<A>,
  repository_provider: Arc<R>,
  config: GitServerConfig,
}

impl<A, R, U> GitServer<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  pub fn new(authenticator: A, repository_provider: R, config: GitServerConfig) -> Self {
    GitServer {
      authenticator: Arc::new(authenticator),
      repository_provider: Arc::new(repository_provider),
      config,
    }
  }

  pub async fn listen(self, port: u16) -> Result<(), GitError> {
    let config = Config {
      inactivity_timeout: Some(std::time::Duration::from_secs(30)),
      auth_rejection_time: std::time::Duration::from_secs(3),
      keys: vec![russh_keys::key::KeyPair::generate_ed25519().unwrap()],
      ..Default::default()
    };
    let config = Arc::new(config);
    let res = russh::server::run(config, ("0.0.0.0", port), self);
    println!("Listening on port {}", port);
    res.await.map_err(|e| e.into())
  }
}

impl<A, R, U> Server for GitServer<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  type Handler = RequestHandler<A, R, U>;

  fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
    RequestHandler::new(
      self.authenticator.clone(),
      self.repository_provider.clone(),
      self.config.clone(),
      peer_addr,
    )
  }
}
