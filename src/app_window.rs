use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

use gtk::prelude::*;

use crate::app::*;

pub struct AppWindow {
    window: gtk::Window,
}

impl AppWindow {
    pub fn new(state: Rc<RefCell<AppState>>, app: App) -> AppWindow {
        let glade_str = include_str!("app_window.glade");
        let builder = gtk::Builder::from_string(glade_str);

        let window: gtk::Window = builder.object("app_window").unwrap();
        let camera_entry: gtk::Entry = builder.object("camera_entry").unwrap();
        let film_entry: gtk::Entry = builder.object("film_entry").unwrap();
        let iso_entry: gtk::Entry = builder.object("iso_entry").unwrap();
        let author_entry: gtk::Entry = builder.object("author_entry").unwrap();
        let comment_buffer: gtk::TextBuffer = builder.object("comment_buffer").unwrap();
        let file_index_checkbox: gtk::CheckButton = builder.object("file_index_checkbox").unwrap();
        let open_button: gtk::Button = builder.object("open_button").unwrap();
        let save_button: gtk::Button = builder.object("save_button").unwrap();
        let files_list_store: gtk::ListStore = builder.object("files_list_store").unwrap();

        let camera_completion_list: gtk::ListStore = builder.object("camera_completion_list").unwrap();
        let film_completion_list: gtk::ListStore = builder.object("film_completion_list").unwrap();
        let iso_completion_list: gtk::ListStore = builder.object("iso_completion_list").unwrap();
        let author_completion_list: gtk::ListStore = builder.object("author_completion_list").unwrap();

        for camera in RefCell::borrow(&state).recent_cameras.iter() {
            camera_completion_list.set(&camera_completion_list.append(), &[(0, &camera)]);
        }

        for film in RefCell::borrow(&state).recent_films.iter() {
            film_completion_list.set(&film_completion_list.append(), &[(0, &film)]);
        }

        for iso in RefCell::borrow(&state).recent_isos.iter() {
            iso_completion_list.set(&iso_completion_list.append(), &[(0, &iso)]);
        }

        for author in RefCell::borrow(&state).recent_authors.iter() {
            author_completion_list.set(&author_completion_list.append(), &[(0, &author)]);
        }


        let window_clone = window.clone();
        let state_clone = Rc::clone(&state);
        let files_list_store_clone = files_list_store.clone();

        open_button.connect_clicked(move |_| {
            let dialog = gtk::FileChooserDialog::with_buttons::<gtk::Window>(Some("Select files"),
                Some(window_clone.borrow()),
                gtk::FileChooserAction::Open,
                &[("_Cancel", gtk::ResponseType::Cancel), ("_Open", gtk::ResponseType::Accept)]);

            let filter = gtk::FileFilter::new();
            filter.add_pattern("*.jpg");
            filter.add_pattern("*.JPG");
            filter.add_pattern("*.jpeg");
            filter.add_pattern("*.JPEG");

            dialog.set_filter(&filter);
            dialog.set_select_multiple(true);

            if dialog.run() == gtk::ResponseType::Accept {
                files_list_store_clone.clear();
                let files = dialog.filenames();

                for pathbuf in files.iter() {
                    state_clone.borrow_mut().files.push(pathbuf.clone().to_path_buf());
                    files_list_store_clone.set(&files_list_store_clone.append(), &[(0,
                        &pathbuf.clone().file_name().unwrap().to_str())]);
                }
                unsafe {
                    dialog.destroy();
                }

            } else {
                unsafe {
                    dialog.destroy();
                }
            }
        });

        let state_clone = Rc::clone(&state);

        save_button.connect_clicked(move |_| {
            let state = &mut state_clone.borrow_mut();

            state.camera = Some(camera_entry.text().as_str().to_string());
            state.film = Some(film_entry.text().as_str().to_string());
            state.iso = Some(iso_entry.text().as_str().to_string());
            state.author = Some(author_entry.text().as_str().to_string());
            state.comment = comment_buffer.text(&comment_buffer.start_iter(),
                &comment_buffer.end_iter(), false).map(|s| { s.as_str().to_string() });

            let result = app.save(state);
        });

        AppWindow {
            window
        }
    }

    pub fn show(&self) {
        glib::set_application_name("Film Taggy");
        self.window.set_wmclass("Film Taggy", "Film Taggy");
        self.window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
        self.window.show_all();
    }
}
