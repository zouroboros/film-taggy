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

    pub fn restore_state(&self) -> Option<AppState> {
        let state = AppState {
            camera: None,
            film: None,
            iso: None,
            author: None,
            comment: None,
            set_file_index: false,
            files: Vec::new(),
            recent_cameras: vec!["Canon EOS 500".to_string(), "Canon EOS 300".to_string(), "Fujica ST 605".to_string()],
            recent_films: vec!["Kodak Tri-X".to_string(), "Kodak T-Max 100".to_string(), "Kodak T-Max 400".to_string(), "Fuji Across II".to_string()],
            recent_isos: vec!["100".to_string(), "200".to_string(), "400".to_string()],
            recent_authors: vec!["Marks".to_string(), "Markx".to_string()]
        };
        return Some(state);
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
