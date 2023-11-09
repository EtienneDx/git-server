use git_server::repository::{Repository, RepositoryPermission};

pub struct SimpleRepository {
  path: String,
}

impl SimpleRepository {
  pub fn new(path: String) -> Self {
    Self { path }
  }
}

impl Repository for SimpleRepository {
  type User = ();

  fn has_permission(&self, _user: &Self::User, _permission: RepositoryPermission) -> bool {
    true
  }

  fn get_path(&self) -> &str {
    &self.path
  }
}
