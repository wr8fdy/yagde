use crate::gd::char;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use anyhow::Result;
use clap::Parser;
use inquire::{Select, Text};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const DEFAULT_PATH: &str = "$HOME/.local/share/Steam/steamapps/compatdata/219990/pfx/drive_c/users/steamuser/Documents/My Games/Grim Dawn/save/main";

#[derive(Parser, Debug)]
#[command(name = "gdr")]
#[command(author = "wr8fdy")]
#[command(about = "Grim Dawn save file editor", version = None, long_about = None)]
struct Cli {
    #[arg(short = 's', long, default_value_t = DEFAULT_PATH.to_string())]
    save_path: String,
}

#[derive(Display, EnumIter, PartialEq, Eq)]
enum CharOpt {
    View,
    Reset,
    Clone,
    Rename,
    #[strum(serialize = "\u{23CE} Return")]
    Return,
    #[strum(serialize = "\u{274C} Exit")]
    Exit,
}

#[derive(Display, EnumIter)]
enum ResetOpt {
    All,
    Skills,
    Attributes,
    Devotions,
    Deaths,
    #[strum(serialize = "\u{23CE} Return")]
    Return,
    #[strum(serialize = "\u{274C} Exit")]
    Exit,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let path = Path::new(&cli.save_path.replace("$HOME", &env::var("HOME")?)).to_path_buf();

    let chars = get_chars_names(path)?;
    let mut options: Vec<&String> = chars.keys().collect();
    let exit = "\u{274C} Exit".to_owned();
    options.sort();
    options.push(&exit);

    'char_select: loop {
        let char = Select::new("Choose character:", options.clone())
            .with_page_size(15)
            .prompt()?;

        if char == &exit {
            break;
        }

        let mut current_char = char::Char::new();
        let file_path = &chars[char];
        current_char.read(file_path)?;

        loop {
            let action = Select::new("Choose an action:", CharOpt::iter().collect())
                .with_page_size(15)
                .prompt()?;

            match action {
                CharOpt::View => current_char.print_info(),
                CharOpt::Rename => {
                    let new_name = Text::new("Enter new name:").prompt()?;
                    current_char.rename(&new_name).save_as(file_path)?;
                }
                CharOpt::Clone => {
                    let new_name = Text::new("Enter new name:").prompt()?;
                    let mut new_char = current_char.clone();
                    new_char
                        .rename(&new_name)
                        .save_as(&Path::new(&("_".to_string() + &new_name)).to_path_buf())?
                }
                CharOpt::Reset => loop {
                    let reset_action = Select::new("Choose an action:", ResetOpt::iter().collect())
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
                },
                CharOpt::Return => break,
                CharOpt::Exit => break 'char_select,
            }
        }
    }

    Ok(())
}

fn get_chars_names(p: PathBuf) -> Result<HashMap<String, PathBuf>> {
    let files = fs::read_dir(&p)?
        .map(|res| res.map(|e| e.path().join("player.gdc")))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut chars = HashMap::new();
    for file in files.iter() {
        let mut c = char::Char::new();
        chars.insert(c.get_name(file)?, file.clone());
    }

    Ok(chars)
}
