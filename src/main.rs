mod action_handler;
use action_handler::handle_action;

fn main() {
    let mut args = std::env::args();

    let kettle_path = "/home/saravenpi/.kettle/";
    handle_action(&mut args, kettle_path);
}
