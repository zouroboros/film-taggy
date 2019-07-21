extern crate gtk;
extern crate glib;
extern crate rexiv2;

use std::rc::Rc;
use std::cell::RefCell;

mod app_window;
mod app;

fn main() {
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");

    let state = Rc::new(RefCell::new(app::AppState {
        camera: None,
        film: None,
        iso: None,
        author: None,
        comment: None,
        set_file_index: false,
        files: Vec::new()
    }));

    let app = app::App::new();

    let app_window = app_window::AppWindow::new(state, app);
    app_window.show();

    gtk::main()
}
