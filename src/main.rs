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

    let app = app::App::new();

    let state = Rc::new(RefCell::new(app.restore_state().expect("Unable to restore app state.")));

    let app_window = app_window::AppWindow::new(state, app);
    app_window.show();

    gtk::main()
}
