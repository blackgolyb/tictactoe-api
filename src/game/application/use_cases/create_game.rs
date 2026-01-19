use crate::game::{
    application::interfaces::GameRepository,
    domain::{
        aggregates::Game,
        models::{GameId, GameRules, Player},
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

    State {
        game_id: GameId,
    }

    Dependencies {
        game_repo: Box<dyn GameRepository>,
    }

    Error {
       InvalidRules,
       GameAlreadyExists,
    };

    Story {
       validate_rules
       generate_game_id
       create_game
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

        generate_game_id |story: &mut Self| {
            let input = story.input();
            // Use hash of the name as game ID
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            input.name.hash(&mut hasher);
            let id = GameId(hasher.finish());

            story.state.game_id = Some(id);
            Ok(())
        }

        create_game |story: &mut Self| {
            let input = story.input();
            let game_id = story.state.game_id.ok_or(Error::InvalidRules)?;

            // Check if game already exists
            if let Some(_) = story.game_repo.get(game_id) {
                return Err(Error::GameAlreadyExists);
            }

            Ok(())
        }

        save_game |story: &mut Self| {
            let input = story.input();
            let game_id = story.state.game_id.ok_or(Error::InvalidRules)?;

            let rules = GameRules::new(input.game_size, input.winning_length);
            let game = Game::new(
                game_id,
                input.name.clone(),
                rules,
                input.first_player,
            );

            story.game_repo.save(&game);
            Ok(())
        }
    }
}
