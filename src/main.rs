use clap::Parser;
use serde_json::Value;
use std::{fs, path::Path};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  path_alias: Option<String>,
  #[clap(short, long, value_parser, default_value = "")]
  alias: String,
}

const CONFIG_FILE_NAME: &str = "config.json";

fn main() {
  let args = Cli::parse();

  let home_path = home::home_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();

  let config_path = format!("{}{}", home_path, "/.config/dira/");

  let config_file_path = format!("{}{}", config_path, CONFIG_FILE_NAME);

  if !Path::new(&config_file_path).is_file() {
    fs::create_dir(config_path).unwrap();
    fs::write(&config_file_path, "{}").unwrap();
  }

  let data = fs::read_to_string(&config_file_path).unwrap();

  let mut config: Value = serde_json::from_str(data.as_str()).unwrap();

  let alias = &args.alias;

  if alias.chars().count() > 0 {
    let cwd = std::env::current_dir()
      .unwrap()
      .into_os_string()
      .into_string()
      .unwrap();
    config[alias] = serde_json::Value::String(cwd);
    let config_as_string = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&config_file_path, config_as_string).unwrap();
    return;
  }

  let path_alias = &args.path_alias.unwrap();

  let path_by_alias = config.get(path_alias).expect("Alias does not exist.");

  let command_str = format!("cd {}", path_by_alias);

  println!("{}", command_str);
}
