use crate::gd::char;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use anyhow::{bail, Ok, Result};
use clap::Parser;
use copy_dir::copy_dir;
use inquire::{Select, Text};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Parser, Debug)]
#[command(name = "yagde")]
#[command(author = "wr8fdy")]
#[command(about = "Grim Dawn save file editor", version = None, long_about = None)]
struct Cli {
    #[arg(short = 's', long)]
    save_path: Option<String>,
}

#[derive(Display, EnumIter, PartialEq, Eq)]
enum CharOpt {
    View,
    Reset,
    Rename,
    Clone,
    AddMoney,
    BoostFrendlyFactions,
    BoostHostileFactions,
    #[strum(serialize = "\u{23CE} Return")]
    Return,
    #[strum(serialize = "\u{274C} Exit")]
    Exit,
}

#[derive(Display, EnumIter)]
enum ResetOpt {
    Deaths,
    Skills,
    Attributes,
    Devotions,
    All,
    #[strum(serialize = "\u{23CE} Return")]
    Return,
    #[strum(serialize = "\u{274C} Exit")]
    Exit,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let path: PathBuf;
    if let Some(p) = &cli.save_path {
        path = Path::new(&p.replace("$HOME", &env::var("HOME")?)).to_path_buf();
    } else {
        path = find_save_files()?;
    }

    'char_select: loop {
        let chars = get_chars_names(&path)?;
        let mut options: Vec<&String> = chars.keys().collect();
        let exit = "\u{274C} Exit".to_owned();
        options.sort();
        options.push(&exit);

        let char = Select::new("Choose character:", options.clone())
            .with_page_size(15)
            .prompt()?;

        if char == &exit {
            break;
        }

        let mut current_char = char::Char::new();
        let current_char_dir = &chars[char];
        let file_path = &current_char_dir.join("player.gdc");
        current_char.read(file_path)?;

        loop {
            let action = Select::new("Choose action:", CharOpt::iter().collect())
                .with_page_size(15)
                .prompt()?;

            match action {
                CharOpt::View => current_char.print_info(),
                CharOpt::Rename => {
                    let new_name = Text::new("Enter a new name:").prompt()?;
                    current_char.rename(&new_name).save_as(file_path)?;
                    println!(
                        "Successfully renamed from {} to {}",
                        current_char.header.name, &new_name
                    );
                }
                CharOpt::Clone => {
                    let new_name = clone_char(&path, current_char_dir)?;
                    println!(
                        "Successfully cloned {} to {}",
                        current_char.header.name, new_name
                    );
                }
                CharOpt::AddMoney => {
                    current_char.info.money = current_char.info.money.saturating_add(10_000_000);
                    current_char.save_as(file_path)?;
                    println!("Current balance is {}", current_char.info.money);
                }
                CharOpt::BoostFrendlyFactions => {
                    current_char.boost_frendly_factions().save_as(file_path)?;
                    println!("Frendly factions are boosted!");
                }
                CharOpt::BoostHostileFactions => {
                    current_char.boost_hostile_factions().save_as(file_path)?;
                    println!("Hostile factions are boosted!");
                }
                CharOpt::Reset => loop {
                    let reset_action = Select::new("Choose action:", ResetOpt::iter().collect())
                        .with_page_size(15)
                        .prompt()?;

                    match reset_action {
                        ResetOpt::All => current_char.reset_all().save_as(file_path)?,
                        ResetOpt::Skills => current_char.reset_skills().save_as(file_path)?,
                        ResetOpt::Attributes => {
                            current_char.reset_attributes().save_as(file_path)?
                        }
                        ResetOpt::Devotions => current_char.reset_devotions().save_as(file_path)?,
                        ResetOpt::Deaths => current_char.reset_deaths().save_as(file_path)?,
                        ResetOpt::Return => break,
                        ResetOpt::Exit => break 'char_select,
                    }
                    println!("Reset {reset_action} done");
                },
                CharOpt::Return => break,
                CharOpt::Exit => break 'char_select,
            }
        }
    }

    Ok(())
}

fn get_chars_names(p: &PathBuf) -> Result<HashMap<String, PathBuf>> {
    let char_dirs = fs::read_dir(p)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut chars = HashMap::new();
    for char_dir in char_dirs.iter() {
        let mut c = char::Char::new();
        let char_file = char_dir.join("player.gdc");
        chars.insert(c.get_name(&char_file)?, char_dir.clone());
    }

    Ok(chars)
}

fn find_save_files() -> Result<PathBuf> {
    let home_dir = &env::var("HOME")?;

    let steam_paths = vec![
        ".local/share/Steam",
        ".steam/debian-installation",
        ".var/app/com.valvesoftware.Steam/data/Steam",
    ];

    let mut steam_path = PathBuf::new();
    for p in steam_paths.iter() {
        steam_path.push(home_dir);
        steam_path.push(p);
        if steam_path.exists() {
            break;
        }
        steam_path.clear();
    }

    let mut path_options: HashMap<String, PathBuf> = HashMap::new();
    add_local_files(&mut path_options, &steam_path)?;
    add_remote_files(&mut path_options, &steam_path)?;

    if path_options.is_empty() {
        bail!("could not detect save files location")
    }

    let save_files_path =
        Select::new("Choose save files location:", path_options.keys().collect()).prompt()?;

    Ok(path_options[save_files_path].clone())
}

fn add_local_files(
    path_options: &mut HashMap<String, PathBuf>,
    steam_path: &PathBuf,
) -> Result<()> {
    const STEAM_LOCAL_PATH: &str = r"steamapps/compatdata/219990/pfx/drive_c/users/steamuser/Documents/My Games/Grim Dawn/save/main";

    let mut path = PathBuf::new();

    path.push(steam_path);
    path.push(STEAM_LOCAL_PATH);
    if path.exists() {
        let count = path.read_dir()?.count();
        path_options.insert(format!("local ({count} chars)"), path.clone());
    }

    Ok(())
}

fn add_remote_files(path_options: &mut HashMap<String, PathBuf>, steam_path: &Path) -> Result<()> {
    const STEAM_REMOTE_ACCOUNT_PATH: &str = r"userdata";
    const STEAM_REMOTE_PATH: &str = r"219990/remote/save/main";

    let mut path = PathBuf::new();

    for p in steam_path.join(STEAM_REMOTE_ACCOUNT_PATH).read_dir()? {
        let p = p?;
        path.push(p.path().clone());
        path.push(STEAM_REMOTE_PATH);
        if path.exists() {
            let count = path.read_dir()?.count();
            path_options.insert(
                format!("remote account {:?} ({count} chars)", p.file_name().clone()),
                path.clone(),
            );
        }
        path.clear();
    }

    Ok(())
}

fn clone_char(path: &Path, current_char_dir: &Path) -> Result<String> {
    let to_name = Text::new("Enter a new name:").prompt()?;
    let to_char_dir = path.join(Path::new(format!("_{}", &to_name).as_str()));

    copy_dir(current_char_dir, &to_char_dir)?;

    let mut cloned_char = char::Char::new();
    let file_path = &to_char_dir.join("player.gdc");

    cloned_char.read(file_path)?;
    cloned_char.rename(&to_name).save_as(file_path)?;

    Ok(to_name)
}
