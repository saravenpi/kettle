mod action_handler;
extern crate dirs;

use action_handler::handle_action;

fn main() {
    let mut args = std::env::args();


    match dirs::home_dir() {
        Some(path) => {
            let home_dir: &str = &path.display().to_string();
            let kettle_path = vec![home_dir, "/.kettle/"].concat();
            handle_action(&mut args, &kettle_path);

        },
        None => println!("Impossible to get your home dir!"),
    }
}
