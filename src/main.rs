mod print;
mod strings;
mod types;
mod vars;

fn main() {
    print::run();
    vars::run();
    types::init();
    strings::start();
}
