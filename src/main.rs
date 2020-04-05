extern crate gtk;
extern crate glib;
extern crate rexiv2;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod app_environment;
mod app_window;
mod app;

fn main() -> Result<(), Box<dyn Error>>{
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");

    let environment = app_environment::AppEnvironment::new()?;

    let app = app::App::new(environment);

    let initial_state = app.initial_state()?;

    let state = Rc::new(RefCell::new(initial_state));

    let app_window = app_window::AppWindow::new(state, app);
    app_window.show();

    gtk::main();
    Ok(())
}
