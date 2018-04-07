extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod window;
mod provider;

fn main() {
    window::start()
}