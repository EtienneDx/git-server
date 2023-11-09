pub mod repository_provider;

#[derive(Debug)]
pub enum RepositoryPermission {
  Read,
  Write,
}

pub trait Repository: Sync + Send + 'static {
  type User;

  fn has_permission(&self, user: &Self::User, permission: RepositoryPermission) -> bool;

  fn get_path(&self) -> &str;
}
