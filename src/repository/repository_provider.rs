use crate::repository::Repository;

pub trait RepositoryProvider: Sync + Send + 'static {
  type User;
  type Repository: Repository<User = Self::User>;

  fn find_repository(&self, path: &str) -> Option<Self::Repository>;
}
