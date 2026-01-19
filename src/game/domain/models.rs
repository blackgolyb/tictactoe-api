use super::value_objects::Segment;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    O,
    X,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FieldState {
    Empty,
    Occupied(Player),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Draw,
    Winner(Player, Segment),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameId(pub u64);

#[derive(Clone, Debug)]
pub struct GameMap {
    pub data: Vec<Vec<FieldState>>,
}

impl GameMap {
    pub fn new(w: usize, h: usize) -> Self {
        let data = (0..h).map(|_| vec![FieldState::Empty; w]).collect();
        Self { data }
    }

    pub fn get(&self, x: usize, y: usize) -> FieldState {
        self.data[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, player: Player) -> Result<(), ()> {
        if self.get(x, y) == FieldState::Empty {
            self.data[y][x] = FieldState::Occupied(player);
            return Ok(());
        }
        Err(())
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.data[0].len(), self.data.len());
    }
}

#[derive(Clone, Debug)]
pub struct GameRules {
    pub game_size: (usize, usize),
    pub winning_length: u32,
}

impl GameRules {
    pub fn new(game_size: (usize, usize), winning_length: u32) -> Self {
        let obj = Self {
            game_size,
            winning_length,
        };
        Self::validate(&obj);
        obj
    }

    fn validate(&self) -> bool {
        let max_size = self.game_size.0.max(self.game_size.1) as u32;
        self.winning_length < 1 || self.winning_length > max_size
    }
}

// type Error = String;
// pub type GameResult<T> = Result<T, Error>;

// pub struct AppDependency {
//     pub conn: Arc<Mutex<Connection>>,
// }

// pub type AppState = web::Data<AppDependency>;
