use std::io;
use std::path::PathBuf;
use std::option::Option;

use rexiv2;

use crate::app_environment::*;

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

pub struct App {
    environment: AppEnvironment
}

impl App {
    pub fn new(environment: AppEnvironment) -> App {
        return App {
            environment
        };
    }

    pub fn initial_state(&self) -> Result<AppState, io::Error> {
        self.environment.restore_state()
    }

    pub fn save(&self, state: &mut AppState) -> Result<AppState, io::Error> {

         if let Some(camera) = &state.camera {
             if(!state.recent_cameras.contains(camera)){
                state.recent_cameras.push(camera.to_string());
             }
        }

        if let Some(film) = &state.film {
            if(!state.recent_films.contains(film)) {
                state.recent_films.push(film.to_string());
            }
        }

        if let Some(iso) = &state.iso {
            if(!state.recent_isos.contains(iso)) {
                state.recent_isos.push(iso.to_string());
            }
        }

        if let Some(author) = &state.author {
            if(!state.recent_authors.contains(author)) {
                state.recent_authors.push(author.to_string());
            }
        }

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

            metadata.save_to_file(file);
        }

        self.environment.save_state(state)?;

        return Ok(state.clone());
    }
}
