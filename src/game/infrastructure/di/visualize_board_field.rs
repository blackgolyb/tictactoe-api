use crate::game::{
    application::use_cases::visualize_board_field::UseCase,
    infrastructure::di::container::DIContainer,
};

pub fn resolve_visualize_board_field_use_case(container: &DIContainer) -> UseCase {
    let game_repo = container.game_repository();
    let visualizer = container.visualize_service();
    UseCase::new(game_repo, visualizer)
}
