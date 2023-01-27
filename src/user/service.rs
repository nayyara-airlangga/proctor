use super::UserRepository;

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
}
