use std::io::*;
use std::fs::*;
use std::path::*;

use glib::get_user_config_dir;

use crate::app::*;

pub struct AppEnvironment {
    recent_dir: PathBuf
}

fn recent_dir() -> PathBuf {
    let mut recent_dir = PathBuf::new();

    #[cfg(not(debug_assertions))]
    recent_dir.push(get_user_config_dir().expect("No user config dir found!"));

    recent_dir.push(".film_taggy");

    recent_dir
}

impl AppEnvironment {
    pub fn new() -> AppEnvironment {

        let dir = recent_dir();

        if(!dir.exists()) {
            AppEnvironment::init_recents_dir();
        }

        return AppEnvironment {
            recent_dir: recent_dir()
        };
    }

    fn init_recents_dir() -> std::io::Result<()> {
        create_dir(recent_dir());

        let recent_cameras = recent_dir().join(Path::new("cameras"));
        let mut recent_cameras_file = File::create(recent_cameras)?;

        let recent_films = recent_dir().join(Path::new("films"));
        let mut recent_films_file = File::create(recent_films)?;

        let recent_isos = recent_dir().join(Path::new("isos"));
        let mut recent_isos_file = File::create(recent_isos)?;
        recent_isos_file.write_all(include_str!("setup/isos").as_bytes());

        let recent_authors = recent_dir().join(Path::new("authors"));
        let mut recent_authors_file = File::create(recent_authors)?;

        Ok(())
    }

    fn read_recent(&self, list: &str) -> std::io::Result<Vec<String>> {
        let file = File::open(recent_dir().join(Path::new(&list)))?;
        BufReader::new(file).lines().collect()
    }

    pub fn restore_state(&self) -> Option<AppState> {

        let recent_cameras = self.read_recent("cameras").expect("Failed to read recent cameras");
        let recent_films = self.read_recent("films").expect("Failed to read recent films");
        let recent_isos = self.read_recent("isos").expect("Failed to read recent isos");
        let recent_authors = self.read_recent("authors").expect("Failed to read recent authors");

        let state = AppState {
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
        };
        return Some(state);
    }
}
