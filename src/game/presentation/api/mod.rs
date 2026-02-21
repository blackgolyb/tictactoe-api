pub mod dtos;
pub mod handlers;
pub mod responses;
pub mod state;

// Re-export commonly used items
pub use handlers::{create_game, get_current_player, get_field, main_page, update_field};
pub use state::AppState;
