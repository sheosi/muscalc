mod calcs;
mod gui;

use gui::Win;
use relm::Widget;

fn main() {
    Win::run(()).unwrap();
}
