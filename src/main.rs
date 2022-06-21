mod action_handler;
use action_handler::handle_action;

fn main() {
    let mut args = std::env::args();

    const KETTLE_PATH: &str = "~/.kettle/";
    handle_action(&mut args, KETTLE_PATH);
}
