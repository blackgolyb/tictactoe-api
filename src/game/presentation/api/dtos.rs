use crate::game::domain::value_objects::Point;

/// Data Transfer Object for field coordinates
#[derive(Debug, Clone, Copy)]
pub struct FieldDto {
    pub x: u64,
    pub y: u64,
}

impl From<FieldDto> for Point {
    fn from(dto: FieldDto) -> Self {
        (dto.x, dto.y)
    }
}

impl From<Point> for FieldDto {
    fn from(point: Point) -> Self {
        Self {
            x: point.0,
            y: point.1,
        }
    }
}

/// Data Transfer Object for game room identifier
#[derive(Debug, Clone)]
pub struct RoomDto {
    pub game_name: String,
}

/// Query parameter for redirect URL
#[derive(Debug)]
pub struct RedirectQuery {
    pub r: Option<String>,
}
