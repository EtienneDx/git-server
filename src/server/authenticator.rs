use crate::error::GitError;

use russh_keys::key::PublicKey;

pub trait Authenticator: Sync + Send + 'static {
  type User: Sync + Send;

  fn validate_public_key(
    &self,
    user: &str,
    key: &PublicKey,
  ) -> Result<Option<Self::User>, GitError>;
}
