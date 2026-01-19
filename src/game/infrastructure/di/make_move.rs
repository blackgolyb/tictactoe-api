use crate::game::{
    application::use_cases::make_move::UseCase, infrastructure::di::container::DIContainer,
};

pub fn resolve_make_move_use_case(container: &DIContainer) -> UseCase {
    let game_repo = container.game_repository();
    UseCase::new(game_repo)
}
