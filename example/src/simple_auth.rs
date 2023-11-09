use git_server::authenticator::Authenticator;

use russh_keys::key::PublicKey;

pub struct SimpleAuth;

impl Authenticator for SimpleAuth {
  type User = ();

  fn validate_public_key(
    &self,
    _user: &str,
    _key: &PublicKey,
  ) -> Result<Option<Self::User>, git_server::error::GitError> {
    Ok(Some(()))
  }
}
