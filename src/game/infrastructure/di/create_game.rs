use crate::game::{
    application::use_cases::create_game::UseCase, infrastructure::di::container::DIContainer,
};

pub fn resolve_create_game_use_case(container: &DIContainer) -> UseCase {
    let game_repo = container.game_repository();
    UseCase::new(game_repo)
}
