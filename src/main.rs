use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Entry;
use gtk::Grid;
use gtk::Label;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "jp.gonypage.tax_calculator_gtk_rs";

struct ApplicationWindowState {
    price_entry: Rc<Entry>,
    tax_entry: Rc<Entry>,
    total_entry: Rc<Entry>,
}

impl ApplicationWindowState {
    fn new(price_entry: &Rc<Entry>, tax_entry: &Rc<Entry>, total_entry: &Rc<Entry>) -> Self {
        Self {
            price_entry: price_entry.clone(),
            tax_entry: tax_entry.clone(),
            total_entry: total_entry.clone(),
        }
    }

    fn calculate(&self) {
        let price = self.price_entry.text();
        if let Ok(price) = price.parse::<i32>() {
            let tax = ((price as f32) * 0.1f32) as i32;
            let total = price + tax;
            self.tax_entry.set_text(&tax.to_string());
            self.total_entry.set_text(&total.to_string());
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
    let entry = Entry::builder().hexpand(true).build();
    let price_entry = Rc::new(entry);
    let entry = Entry::builder().build();
    let tax_entry = Rc::new(entry);
    let entry = Entry::builder().build();
    let total_entry = Rc::new(entry);

    let price_label = Label::builder().label("Price").build();
    let tax_label = Label::builder().label("Tax").build();
    let total_label = Label::builder().label("Total").build();
    let grid = Grid::builder().build();
    grid.attach(&price_label, 0, 0, 1, 1);
    grid.attach(price_entry.as_ref(), 1, 0, 1, 1);
    grid.attach(&tax_label, 0, 1, 1, 1);
    grid.attach(tax_entry.as_ref(), 1, 1, 1, 1);
    grid.attach(&total_label, 0, 2, 1, 1);
    grid.attach(total_entry.as_ref(), 1, 2, 1, 1);
    grid.attach(calculate_button.as_ref(), 0, 3, 2, 1);

    let state = Rc::new(RefCell::new(ApplicationWindowState::new(
        &price_entry,
        &tax_entry,
        &total_entry,
    )));
    calculate_button.connect_clicked(clone!(
        #[strong]
        state,
        move |_| {
            let state1 = state.borrow();
            state1.calculate();
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("tax_calculator_gtk_rs")
        .child(&grid)
        .build();
    window.present();
}
