use crate::game::{
    application::interfaces::GameRepository,
    domain::{aggregates::Game, value_objects::Point},
};
use crate::use_case;

use_case! {
    Input {
        name: String,
        field: Point,
    }

    State {
        game: Game,
    }

    Dependencies {
        game_repo: Box<dyn GameRepository>,
    }

    Error {
       Fail,
    };

    Story {
       find_game
       make_move
    }

    Steps {
        find_game |story: &mut Self| {
            let input = story.input();
            let name = &input.name;

            let game = story.game_repo.get_by_name(name).ok_or(Error::Fail)?;

            story.state.game = Some(game);
            Ok(())
        }

        make_move |story: &mut Self| {
            let input = story.input();
            let field = input.field;
            let game = story.state.game.as_mut().ok_or(Error::Fail)?;

            game.make_move(field.0 as usize, field.1 as usize)
                .or(Err(Error::Fail))?;

            story.game_repo.save(game);
            Ok(())
        }
    }
}
