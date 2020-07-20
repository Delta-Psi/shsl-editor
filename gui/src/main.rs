use gtk::{self, prelude::*};
use gio::prelude::*;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    window.set_title("SHSL Editor");

    window.show_all();
}

fn main() {
    let app = gtk::Application::new(None, Default::default()).expect("could not initialize app");

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run(&std::env::args().collect::<Vec<_>>());
}
