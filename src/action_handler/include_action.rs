use std::{fs, io::Write};
use std::fs::File;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    imported_files: Vec<String>,
    kettle_path: String,
}

pub fn handle_action(file_name: &str) {
    if Path::new(file_name).exists() {

        let kettle_recipe = fs::read_to_string("recipe.json")
            .expect("Error encountered while reading the recipe file");

        let mut recipe_json: Recipe = serde_json::from_str(&kettle_recipe)
            .expect("Error encountered while deserialising the json recipe");

        let included_file_name = String::from(file_name);

        recipe_json.imported_files.push(included_file_name);

        let mut recipe_file = File::create("recipe.json")
            .expect("Error while writing to file");

        let new_recipe_json = serde_json::to_string_pretty(&recipe_json)
            .unwrap();

        recipe_file.write_all(new_recipe_json.as_bytes())
            .expect("Error while writing to file");

        println!("✅ {file_name} successfully included !");
    } else {
        println!("⚠️  This file doesn't exist");
    }
}