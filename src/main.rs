mod ui;

use parsefile_lib::{init_subject, parse_file};

use ui::hello;

fn main() {
    init_subject();

    hello();
}
