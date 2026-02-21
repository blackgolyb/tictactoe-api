use crate::game::{
    application::interfaces::GameRepository,
    domain::models::GameRules,
};
use crate::use_case;

use_case! {
    Input {
        name: String,
    }

    State {
        rules: GameRules,
    }

    Dependencies {
        game_repo: Box<dyn GameRepository>,
    }

    Error {
       GameNotFound,
    };

    Story {
       find_game_and_extract_rules
    }

    Steps {
        find_game_and_extract_rules |story: &mut Self| {
            let input = story.input();
            let name = &input.name;

            let game = story.game_repo.get_by_name(name).ok_or(Error::GameNotFound)?;

            story.state.rules = Some(game.rules().clone());
            Ok(())
        }
    }
}
