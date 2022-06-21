# Kettle 🫖
## The boilerplate manager

### How it works:

```
Welcome to Kettle 🫖
the boilerplate manager

COMMANDS:
  help, --h, --help                      : Shows this message
  init <kettle_name>                     : Initialises a kettle in your current folder
  include <file_name>                    : Includes the given file to the recipe.json file
  exclude <file_name>                    : Excludes the given file from the recipe.json file
  save                                   : Saves the kettle to the kettle repo
  delete <kettle_name>                   : Deletes a kettle from the kettle repo
  use <kettle_name> <destination_folder> : Import a kettle to the destination_folder
  list                                   : Lists all the kettles in the kettle repo
```

### Installation:

Your can run:
```
./install.sh
```
That will run itself:
```
cargo build --release
sudo cp target/release/kettle /usr/bin
```

If the install file doesn't execute, run this before:
```
chmod +x install.sh
```
