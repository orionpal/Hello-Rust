// // importing other files as modules using keyword mod
// mod hello;
// // The function called main will run when the compiled .exe file is called
// fn main() {
//     hello::hello();
// }

extern crate gtk;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = 
        Rc::new(
        gtk::Window::new(
        gtk::WindowType::Toplevel));
    let weak_window = Rc::downgrade(&window);
    window.set_title("Window with button");
    window.set_default_size(350, 70);

    let button = gtk::Button::new_with_label("Create another button");
    
    button.connect_clicked(move |_| {
        let new_button = gtk::Button::new_with_label("New button");
        if let Some(window) = weak_window.upgrade() {
            window.add(&new_button);
            new_button.show();
        }
    });

    window.add(&button);
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}