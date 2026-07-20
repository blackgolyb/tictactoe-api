use crate::game::{
    application::{
        interfaces::{GameRepository, ImageSize},
        services::visualize_service::VisualizeService,
    },
    domain::{aggregates::Game, value_objects::Point},
};
use crate::use_case;

use_case! {
    Input {
        name: String,
        field: Point,
        size: Option<ImageSize>,
    }

    State {
        game: Game,
        field_img: Vec<u8>,
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
       visualize_field
    }

    Steps {
        find_game |story: &mut Self| {
            let input = story.input();
            let name = &input.name;

            let game = story.game_repo.get_by_name(name).ok_or(Error::Fail)?;

            story.state.game = Some(game);
            Ok(())
        }

        visualize_field |story: &mut Self| {
            let input = story.input();
            let field = input.field;
            let size = input.size;
            let game = story.state.game.as_ref().ok_or(Error::Fail)?;

            let field_img = story.visualizer.visualize_field(game, field, size).or(Err(Error::Fail))?;

            story.state.field_img = Some(field_img);
            Ok(())
        }
    }
}
