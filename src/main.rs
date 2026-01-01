mod ui;

use parsefile_lib::monitor;
use parsefile_lib::parse_file;

use ui::hello;

fn main() {
    monitor();
    hello();
}
