use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn main() {
    let application = Application::builder()
        .application_id("com.danielm.HelloWorld")
        .build();

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rusty GTK4")
        .default_width(450)
        .default_height(100)
        .build();

    let label = Label::with_mnemonic("_Hello Rusty GTK4");
    window.set_child(Some(&label));

    window.present();
}
