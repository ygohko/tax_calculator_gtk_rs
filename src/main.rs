use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow,  Button};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "jp.gonypage.tax_calculator_gtk_rs";

struct ApplicationWindowState {
    calculate_button: Rc<Button>,
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Calcluate")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let calculate_button = Rc::new(button);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("tax_calculator_gtk_rs")
        .child(calculate_button.as_ref())
        .build();
    window.present();
}
