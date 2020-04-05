use std::io;
use std::io::Write;
use std::io::BufRead;
use std::fs;
use std::path;

use glib::get_user_config_dir;

use crate::app::*;

pub struct AppEnvironment {
    recent_dir: path::PathBuf
}

impl AppEnvironment {
    pub fn new() -> Result<AppEnvironment, io::Error> {

        let dir = AppEnvironment::recent_dir();

        if !dir.exists() {
            AppEnvironment::init_recents_dir()?;
        }

        Ok(AppEnvironment {
            recent_dir: AppEnvironment::recent_dir()
        })
    }

    fn recent_dir() -> path::PathBuf {
        let mut recent_dir = path::PathBuf::new();

        #[cfg(not(debug_assertions))]
        recent_dir.push(get_user_config_dir().expect("No user config dir found!"));

        recent_dir.push("film_taggy");

        recent_dir
    }

    fn init_recents_dir() -> Result<(), io::Error> {
        fs::create_dir(AppEnvironment::recent_dir())?;

        let dir = AppEnvironment::recent_dir();

        let recent_cameras = dir.join(path::Path::new("cameras"));
        let _recent_cameras_file = fs::File::create(recent_cameras)?;

        let recent_films = dir.join(path::Path::new("films"));
        let _recent_films_file = fs::File::create(recent_films)?;

        let recent_isos = dir.join(path::Path::new("isos"));
        let mut recent_isos_file = fs::File::create(recent_isos)?;
        recent_isos_file.write_all(include_str!("setup/isos").as_bytes())?;

        let recent_authors = dir.join(path::Path::new("authors"));
        let _recent_authors_file = fs::File::create(recent_authors)?;

        Ok(())
    }

    fn read_recent(&self, list: &str) -> Result<Vec<String>, io::Error> {
        let file = fs::File::open(self.recent_dir.join(path::Path::new(&list)))?;
        io::BufReader::new(file).lines().collect()
    }

    pub fn restore_state(&self) -> Result<AppState, io::Error> {

        let recent_cameras = self.read_recent("cameras")?;
        let recent_films = self.read_recent("films")?;
        let recent_isos = self.read_recent("isos")?;
        let recent_authors = self.read_recent("authors")?;

        Ok(AppState {
            camera: None,
            film: None,
            iso: None,
            author: None,
            comment: None,
            set_file_index: false,
            files: Vec::new(),
            recent_cameras: recent_cameras,
            recent_films: recent_films,
            recent_isos: recent_isos,
            recent_authors: recent_authors
        })
    }

    fn save_recent(&self, list: &str, entries: &Vec<String>) -> Result<(), io::Error> {
        fs::write(self.recent_dir.join(path::Path::new(&list)), entries.join("\n"))
    }

    pub fn save_state(&self, state: &AppState) -> Result<(), io::Error> {
        self.save_recent("cameras", &state.recent_cameras)?;
        self.save_recent("films", &state.recent_films)?;
        self.save_recent("isos", &state.recent_isos)?;
        self.save_recent("authors", &state.recent_authors)?;

        Ok(())
    }
}
