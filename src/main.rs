use clap::Parser;

use clipboard::{ClipboardContext, ClipboardProvider};
use serde_json::Value;
use std::{fs, path::Path};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  /// Alias of the directory to navigate to
  path_alias: Option<String>,
  /// Alias to assign to current working directory
  #[clap(short, long, value_parser, default_value = "")]
  alias: String,
  /// Should list directory aliases
  #[clap(short, long, value_parser, default_value = "false")]
  list: bool,
}

const CONFIG_FILE_NAME: &str = "config.json";

fn get_or_build_config_dir() -> String {
  let home_path = home::home_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();

  let config_path = format!("{}{}", home_path, "/.config/dira/");

  let config_file_path = format!("{}{}", config_path, CONFIG_FILE_NAME);

  if !Path::new(&config_path).is_dir() {
    fs::create_dir(config_path).unwrap();
  }

  if !Path::new(&config_file_path).is_file() {
    fs::write(&config_file_path, "{}").unwrap();
  }

  return config_file_path;
}

fn main() -> Result<(), String> {
  let args = Cli::parse();

  let config_file_path = get_or_build_config_dir();

  let config = fs::read_to_string(&config_file_path).unwrap();

  if args.list {
    println!("{}", config);
    return Ok(());
  }

  let mut mutable_config: Value = serde_json::from_str(config.as_str()).unwrap();

  let alias = args.alias;

  if alias.chars().count() > 0 {
    let cwd = std::env::current_dir()
      .unwrap()
      .into_os_string()
      .into_string()
      .unwrap();
    mutable_config[alias] = serde_json::Value::String(cwd);
    let config_as_string = serde_json::to_string_pretty(&mutable_config).unwrap();
    fs::write(&config_file_path, config_as_string).unwrap();
    return Ok(());
  }

  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

  let path_alias = args.path_alias.unwrap();

  let path_by_alias = match mutable_config.get(path_alias) {
    Some(path) => Ok(path),
    None => Err("Alias does not exist"),
  };

  let command_str = format!("cd {}", path_by_alias?);

  ctx.set_contents(command_str.to_owned()).unwrap();

  println!("{} added to clipboard", command_str);

  Ok(())
}
