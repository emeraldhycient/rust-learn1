mod arrays;
mod print;
mod strings;
mod tuples;
mod types;
mod vars;

fn main() {
    print::run();
    vars::run();
    types::init();
    strings::start();
    tuples::init();
    arrays::start();
}
