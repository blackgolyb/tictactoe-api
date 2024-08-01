use std::path::PathBuf;

use crate::core::types::{FieldId, FieldStatus, LineType, WinnerSequence};

use super::traits::VisualizeGame;

pub struct GameVisualizeService {
    assets_folder: PathBuf,
}

impl VisualizeGame for GameVisualizeService {
    fn get_field_image(
        &self,
        field_id: FieldId,
        field_status: FieldStatus,
        winners_field: Option<WinnerSequence>,
    ) -> Vec<u8> {
        std::fs::read(self.get_filed_image_path(field_id, field_status, winners_field)).unwrap()
    }
}

impl GameVisualizeService {
    pub fn new<P: Into<PathBuf>>(assets_folder: P) -> Self {
        Self {
            assets_folder: assets_folder.into(),
        }
    }

    fn get_line_type(winners_field: WinnerSequence) -> LineType {
        let is_vertical = winners_field.windows(2).all(|w| w[0] % 3 == w[1] % 3);
        if is_vertical {
            return LineType::Vertical;
        };

        let is_horizontal = winners_field.windows(2).all(|w| w[0] / 3 == w[1] / 3);
        if is_horizontal {
            return LineType::Horizontal;
        };

        if winners_field.contains(&0) {
            LineType::Diagonal1
        } else {
            LineType::Diagonal2
        }
    }

    fn get_filed_image_path(
        &self,
        field_id: FieldId,
        field_status: FieldStatus,
        winners_field: Option<WinnerSequence>,
    ) -> PathBuf {
        let line_postfix = winners_field
            .as_ref()
            .filter(|w| w.contains(&field_id))
            .and_then(|w| {
                Some(match Self::get_line_type(w.to_vec()) {
                    LineType::Vertical => "_v",
                    LineType::Horizontal => "_h",
                    LineType::Diagonal1 => "_d1",
                    LineType::Diagonal2 => "_d2",
                })
            })
            .unwrap_or("");

        let path = match field_status {
            FieldStatus::Empty => "Empty.png".to_string(),
            FieldStatus::O => format!("O{}.png", line_postfix),
            FieldStatus::X => format!("X{}.png", line_postfix),
        };

        self.assets_folder.join(path)
    }
}
