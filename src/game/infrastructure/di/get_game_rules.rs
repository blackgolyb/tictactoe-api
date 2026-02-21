use crate::game::{
    application::use_cases::get_game_rules::UseCase, infrastructure::di::container::DIContainer,
};

pub fn resolve_get_game_rules_use_case(container: &DIContainer) -> UseCase {
    let game_repo = container.game_repository();
    UseCase::new(game_repo)
}
