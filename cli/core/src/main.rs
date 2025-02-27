pub use anyhow::Result;
use clap::{crate_version, load_yaml, App, AppSettings, ArgMatches};
use dialoguer::Input;

mod build;
mod dev;
mod helpers;
mod info;
mod init;

pub use helpers::Logger;

macro_rules! value_or_prompt {
  ($init_runner: ident, $setter_fn: ident, $value: ident, $ci: ident, $prompt_message: expr) => {{
    let mut init_runner = $init_runner;
    if let Some(value) = $value {
      init_runner = init_runner.$setter_fn(value);
    } else if !$ci {
      let input = Input::<String>::new()
        .with_prompt($prompt_message)
        .interact_text()?;
      init_runner = init_runner.$setter_fn(input);
    }
    init_runner
  }};
}

fn init_command(matches: &ArgMatches) -> Result<()> {
  let force = matches.is_present("force");
  let directory = matches.value_of("directory");
  let tauri_path = matches.value_of("tauri-path");
  let app_name = matches.value_of("app-name");
  let window_title = matches.value_of("window-title");
  let dist_dir = matches.value_of("dist-dir");
  let dev_path = matches.value_of("dev-path");
  let ci = matches.is_present("ci") || std::env::var("CI").is_ok();

  let mut init_runner = init::Init::new();
  if force {
    init_runner = init_runner.force();
  }
  if let Some(directory) = directory {
    init_runner = init_runner.directory(directory);
  }
  if let Some(tauri_path) = tauri_path {
    init_runner = init_runner.tauri_path(tauri_path);
  }
  init_runner = value_or_prompt!(
    init_runner,
    app_name,
    app_name,
    ci,
    "What is your app name?"
  );
  init_runner = value_or_prompt!(
    init_runner,
    window_title,
    window_title,
    ci,
    "What should the window title be?"
  );
  init_runner = value_or_prompt!(
    init_runner,
    dist_dir,
    dist_dir,
    ci,
    r#"Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri" folder that will be created?"#
  );
  init_runner = value_or_prompt!(
    init_runner,
    dev_path,
    dev_path,
    ci,
    "What is the url of your dev server?"
  );

  init_runner.run()
}

fn dev_command(matches: &ArgMatches) -> Result<()> {
  let exit_on_panic = matches.is_present("exit-on-panic");
  let config = matches.value_of("config");

  let mut dev_runner = dev::Dev::new().exit_on_panic(exit_on_panic);

  if let Some(config) = config {
    dev_runner = dev_runner.config(config.to_string());
  }

  dev_runner.run()
}

fn build_command(matches: &ArgMatches) -> Result<()> {
  let debug = matches.is_present("debug");
  let verbose = matches.is_present("verbose");
  let targets = matches.values_of_lossy("target");
  let config = matches.value_of("config");

  let mut build_runner = build::Build::new();
  if debug {
    build_runner = build_runner.debug();
  }
  if verbose {
    build_runner = build_runner.verbose();
  }
  if let Some(targets) = targets {
    build_runner = build_runner.targets(targets);
  }
  if let Some(config) = config {
    build_runner = build_runner.config(config.to_string());
  }

  build_runner.run()
}

fn info_command() -> Result<()> {
  info::Info::new().run()
}

fn main() -> Result<()> {
  let yaml = load_yaml!("cli.yml");
  let app = App::from(yaml)
    .version(crate_version!())
    .setting(AppSettings::ArgRequiredElseHelp)
    .setting(AppSettings::GlobalVersion)
    .setting(AppSettings::SubcommandRequired);
  let app_matches = app.get_matches();
  let matches = app_matches.subcommand_matches("tauri").unwrap();

  if let Some(matches) = matches.subcommand_matches("init") {
    init_command(&matches)?;
  } else if let Some(matches) = matches.subcommand_matches("dev") {
    dev_command(&matches)?;
  } else if let Some(matches) = matches.subcommand_matches("build") {
    build_command(&matches)?;
  } else if matches.subcommand_matches("info").is_some() {
    info_command()?;
  }

  Ok(())
}
