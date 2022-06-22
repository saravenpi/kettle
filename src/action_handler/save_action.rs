use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, vec};

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    imported_files: Vec<String>,
    kettle_path: String,
}

pub fn handle_action(kettle_repo_path: &str) {
    if Path::new("recipe.json").exists() {
        let kettle_recipe = fs::read_to_string("recipe.json")
            .expect("Error encountered while reading the recipe file");

        let recipe_json: Recipe = serde_json::from_str(&kettle_recipe)
            .expect("Error encountered while deserialising the json recipe");

        let kettle_path = vec![kettle_repo_path.to_string(), recipe_json.name].concat();

        for file_name in recipe_json.imported_files {
            let kettle_repo_file =
                vec![kettle_path.to_string(), "/".to_string(), file_name.clone()].concat();

            fs::copy(file_name, kettle_repo_file)
                .expect("Error encountered copying files from repo to the destination folder");
        }
        println!("✅ kettle successfully saved !");
    } else {
        println!("⚠️  initialise a kettle first")
    }
}
