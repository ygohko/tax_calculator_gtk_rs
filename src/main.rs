use glib::clone;
use gtk::prelude::*;
use gtk::glib;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Entry;
use gtk::Orientation;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "jp.gonypage.tax_calculator_gtk_rs";

struct ApplicationWindowState {
    price_entry: Rc<Entry>,
    tax_entry: Rc<Entry>,
    total_entry: Rc<Entry>,
    calculate_button: Rc<Button>,
}

impl ApplicationWindowState {
    fn new(price_entry: &Rc<Entry>, tax_entry: &Rc<Entry>, total_entry: &Rc<Entry>, calculate_button: &Rc<Button>) -> Self {
        Self {
            price_entry: price_entry.clone(),
            tax_entry: tax_entry.clone(),
            total_entry: total_entry.clone(),
            calculate_button: calculate_button.clone(),
        }
    }
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
    let entry = Entry::builder()
        .build();
    let price_entry = Rc::new(entry);
    let entry = Entry::builder()
        .build();
    let tax_entry = Rc::new(entry);
    let entry = Entry::builder()
        .build();
    let total_entry = Rc::new(entry);

    let box1 = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    box1.append(price_entry.as_ref());
    box1.append(tax_entry.as_ref());
    box1.append(total_entry.as_ref());
    box1.append(calculate_button.as_ref());

    let state = Rc::new(RefCell::new(ApplicationWindowState::new(
        &price_entry,
        &tax_entry,
        &total_entry,
        &calculate_button,
    )));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("tax_calculator_gtk_rs")
        .child(&box1)
        .build();
    window.present();
}
