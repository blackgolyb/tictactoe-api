use crate::game::{
    application::{interfaces::GameRepository, services::visualize_service::VisualizeService},
    domain::aggregates::Game,
};
use crate::use_case;

use_case! {
    Input {
        name: String,
        size: Option<(u32, u32)>,
    }

    State {
        game: Game,
        current_player_img: Vec<u8>,
    }

    Dependencies {
        game_repo: Box<dyn GameRepository>,
        visualizer: VisualizeService,
    }

    Error {
       Fail,
    };

    Story {
       find_game
       visualize_current_player
    }

    Steps {
        find_game |story: &mut Self| {
            let input = story.input();
            let name = &input.name;

            let game = story.game_repo.get_by_name(name).ok_or(Error::Fail)?;

            story.state.game = Some(game);
            Ok(())
        }

        visualize_current_player |story: &mut Self| {
            let input = story.input();
            let game = story.state.game.as_ref().ok_or(Error::Fail)?;

            let current_player_img = story.visualizer.visualize_current_player(game, input.size).or(Err(Error::Fail))?;

            story.state.current_player_img = Some(current_player_img);
            Ok(())
        }
    }
}
