mod print;
mod vars;
mod types;
mod strings;
// imports the print.r file

fn main() {
    //calls and runs the print.rs file
    print::run();
    vars::run();
    types::run();
    strings::run();
}
