mod save_action;
mod delete_action;
mod init_action;
mod list_action;
mod use_action;
mod exclude_action;
mod include_action;

fn check_default(unwraped_data: &str, message: &str) -> bool {
    if unwraped_data == "" {
        println!("{message}");
        return false
    } else {
        return true;
    }
}
pub fn handle_action(args: &mut std::env::Args, kettle_repo_path: &str) {

    let action = args.nth(1).unwrap_or_default();
    if action == "" {
        println!("Welcome to Kettle, use -h for all the commands");
    } else {
        let action_args = args.into_iter();

        if action == "save" {
            save_action::handle_action(kettle_repo_path);
        } else if action == "delete" {
            let kettle_name = action_args.nth(0).unwrap_or_default();
            if check_default(&kettle_name, "No kettle name was given") {
                delete_action::handle_action(&kettle_name, kettle_repo_path);
            }
        } else if action == "init" {
            let kettle_name = action_args.nth(0).unwrap_or_default();
            if check_default(&kettle_name, "No kettle name was given") {
                init_action::handle_action(&kettle_name, kettle_repo_path);
            }
        } else if action == "list" {
            list_action::handle_action(kettle_repo_path);
        } else if action == "include" {
            let file_name = args.nth(0).unwrap_or_default();
            if check_default(&file_name, "No file name was given") {
                include_action::handle_action(&file_name);
            }
        } else if action == "exclude" {
            let file_name = args.nth(0).unwrap_or_default();
                if check_default(&file_name, "No file name was given") {
                    exclude_action::handle_action(&file_name);
                }
        } else if action == "use" {
            let kettle_name = action_args.nth(0).unwrap_or_default();
            if check_default(&kettle_name, "No kettle name was given") {
                let destination_folder = action_args.nth(0).unwrap_or_default();
                if check_default(&destination_folder, "No destination folder was given") {
                    use_action::handle_action(&kettle_name, &destination_folder, kettle_repo_path);
                }
            }
        } else if action == "-h" || action == "--help" || action == "help" {
            println!(" * Welcome to Kettle 🫖    * ");
            println!(" * the boilerplate manager *");
            println!("");
            println!("COMMANDS:");
            println!("  help, --h, --help                      : Shows this message");
            println!("  init <kettle_name>                     : Initialises a kettle in your current folder");
            println!("  include <file_name>                    : Includes the given file to the recipe.json file");
            println!("  exclude <file_name>                    : Excludes the given file from the recipe.json file");
            println!("  save                                   : Saves the kettle to the kettle repo");
            println!("  delete <kettle_name>                   : Deletes a kettle from the kettle repo");
            println!("  use <kettle_name> <destination_folder> : Import a kettle to the destination_folder");
            println!("  list                                   : Lists all the kettles in the kettle repo");
            println!("");
            println!("- created with ❤️  by @saravenpi");
            println!("- https://github.com/saravenpi\n");

        } else {
            println!("This command was not found, use -h for all the commands")
        }
    }
}
