use std::path::PathBuf;
use std::option::Option;

use rexiv2::*;

#[derive(Clone)]
pub struct AppState {
    pub camera: Option<String>,
    pub film: Option<String>,
    pub iso: Option<String>,
    pub author: Option<String>,
    pub comment: Option<String>,
    pub set_file_index: bool,
    pub files: Vec<PathBuf>,
    pub recent_cameras: Vec<String>,
    pub recent_films: Vec<String>,
    pub recent_isos: Vec<String>,
    pub recent_authors: Vec<String>
}

pub struct App {}

impl App {
    pub fn new() -> App {
        return App {};
    }

    pub fn save(&self, state: &AppState) -> Result<AppState> {
        for file in state.files.iter() {
            let mut metadata = rexiv2::Metadata::new_from_path(&file).unwrap();

            if let Some(camera) = &state.camera {
                metadata.set_tag_string("Exif.Image.Model", &camera);
            }

            if let Some(film) = &state.film {
                metadata.set_tag_string("Exif.Image.Make", &film);
            }

            if let Some(iso) = &state.iso {
                metadata.set_tag_string("Exif.Photo.ISOSpeedRatings", &iso);
            }

            if let Some(comment) = &state.comment {
                metadata.set_tag_string("Exif.Photo.UserComment", &comment);
            }

            metadata.save_to_file(file).expect("Error writing exif data!");
        }

        return Ok(state.clone());
    }
}
