use crate::game::{
    application::interfaces::GameRepository,
    domain::{
        aggregates::Game,
        models::{GameRules, Player},
    },
};
use crate::use_case;

use_case! {
    Input {
        name: String,
        game_size: (usize, usize),
        winning_length: u32,
        first_player: Player,
    }

    State {}

    Dependencies {
        game_repo: Box<dyn GameRepository>,
    }

    Error {
       InvalidRules,
       GameAlreadyExists,
    };

    Story {
       validate_rules
       check_game_exists
       save_game
    }

    Steps {
        validate_rules |story: &mut Self| {
            let input = story.input();
            let max_size = input.game_size.0.max(input.game_size.1) as u32;

            if input.winning_length < 1 || input.winning_length > max_size {
                return Err(Error::InvalidRules);
            }

            if input.game_size.0 == 0 || input.game_size.1 == 0 {
                return Err(Error::InvalidRules);
            }

            Ok(())
        }

        check_game_exists |story: &mut Self| {
            let input = story.input();

            // Check if game already exists
            if let Some(_) = story.game_repo.get_by_name(&input.name) {
                return Err(Error::GameAlreadyExists);
            }

            Ok(())
        }

        save_game |story: &mut Self| {
            let input = story.input();

            let rules = GameRules::new(input.game_size, input.winning_length);
            let game = Game::new(
                input.name.clone(),
                rules,
                input.first_player,
            );

            story.game_repo.save(&game);
            Ok(())
        }
    }
}
