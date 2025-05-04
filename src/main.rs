/*
 * Copyright (c) 2025 Yasuaki Gohko
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE ABOVE LISTED COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::Align;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Entry;
use gtk::Grid;
use gtk::InputPurpose;
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
    let price_label = Label::builder()
        .label("Price")
        .margin_top(12)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    let tax_label = Label::builder()
        .label("Tax")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();
    let total_label = Label::builder()
        .label("Total")
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    let price_entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(6)
        .margin_end(12)
        .input_purpose(InputPurpose::Digits)
        .xalign(1.0f32)
        .hexpand(true)
        .build();
    let price_entry = Rc::new(price_entry);
    let tax_entry = Entry::builder()
        .margin_top(6)
        .margin_bottom(6)
        .margin_end(12)
        .editable(false)
        .xalign(1.0f32)
        .build();
    let tax_entry = Rc::new(tax_entry);
    let total_entry = Entry::builder()
        .margin_top(6)
        .margin_bottom(6)
        .margin_end(12)
        .editable(false)
        .xalign(1.0f32)
        .build();
    let total_entry = Rc::new(total_entry);
    let button = Button::builder()
        .label("Calculate")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let calculate_button = Rc::new(button);

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
        .title("Tax Calculator")
        .child(&grid)
        .build();
    window.present();
}
