mod output;

use alpac::{Ingredients, RecipeVersion};
use clap::{App, Arg};
use std::fs;
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! die {
    () => {
        std::process::exit(1);
    };
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        die!();
    }};
}

fn main()
{
  let matches = App::new("Alpac - The Almighty Package Manager")
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
    .about(clap::crate_description!())
    .arg(
      Arg::with_name("recipe-dir")
        .short("d")
        .long("recipe-dir")
        .value_name("DIR")
        .help("Directory that contains the recipe")
        .takes_value(true)
        .required(true),
    )
    .arg(
      Arg::with_name("recipe-version")
        .short("r")
        .long("recipe-version")
        .value_name("STRING")
        .help("Which version of the recipe to build")
        .takes_value(true)
        .default_value("latest"),
    )
    .get_matches();

  let recipe_dir: PathBuf = matches.value_of("recipe-dir").unwrap().parse().unwrap();
  if !recipe_dir.is_dir() {
    die!("Given recipe directory ('{}') does not exist.", recipe_dir.display());
  }
  let ingredients_path = recipe_dir.join("ingredients.yaml");
  if !ingredients_path.is_file() {
    die!("There is no 'ingredients.yaml' in '{}'", recipe_dir.display());
  }

  let recipe_version = match matches.value_of("recipe-version").unwrap() {
    "latest" => RecipeVersion::Latest,
    v => RecipeVersion::Version(v.to_string()),
  };

  let ingredients = Ingredients::read(&ingredients_path).unwrap();

  ingredients.get_all_sources().iter().for_each(|s| {
    if s.download_to_dir(".") {
      println!("File was successfully downloaded to `{}`.", s.get_filename());
    } else {
      die!("Downloaded file did not match the expected hashes.");
    }
  });
}
