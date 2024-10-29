use clap::{Arg, ArgAction, Command};
use color_eyre::eyre::{eyre, Result};
use colored::*;
use inquire::{Confirm, Select};
use std::env;
use std::fs;
use std::io::Write;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command as ProcessCommand;

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = Command::new("skip-icloud-sync")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("folder")
                .short('f')
                .long("folder")
                .value_name("NAME")
                .help("Folder to make nosync")
                .default_value("node_modules"),
        )
        .arg(
            Arg::new("skip-git")
                .short('s')
                .long("skip-git")
                .help("Skip adding to .gitignore")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let folder = matches.get_one::<String>("folder").unwrap();
    let skip_git = matches.get_flag("skip-git");

    let pwd = env::current_dir()?;
    let base_path = pwd.join(folder);
    let nosync_path = base_path.with_extension("nosync");

    if check_pwd_is_in_icloud(&pwd)? {
        create_nosync_folder(&base_path, &nosync_path)?;
        create_symlink(&base_path, &nosync_path)?;
        if !skip_git {
            add_gitignore(&pwd, folder)?;
            add_gitignore(&pwd, &format!("{}.nosync", folder))?;
        }
        println!("{}", "Done!".green());
    } else {
        println!("Not in iCloud, exiting.");
        process::exit(1);
    }

    Ok(())
}

fn check_cloud_folder(pwd: &Path) -> Result<bool> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Unable to get home directory"))?;

    let cloud_folder = home_dir
        .join("Library/Mobile Documents/com~apple~CloudDocs")
        .join(pwd.strip_prefix(&home_dir)?);

    if cloud_folder.exists() {
        Ok(true)
    } else {
        let answer = Confirm::new("Current folder is not in iCloud. Continue?").prompt()?;
        Ok(answer)
    }
}

fn check_pwd_is_in_icloud(pwd: &PathBuf) -> Result<bool> {
    let pwd_lower = pwd.to_string_lossy().to_lowercase();
    if pwd_lower.contains("com~apple~clouddocs") {
        Ok(true)
    } else if pwd_lower.contains("desktop") || pwd_lower.contains("documents") {
        check_cloud_folder(pwd)
    } else {
        let answer = Confirm::new("Current folder is not in iCloud. Continue?").prompt()?;
        Ok(answer)
    }
}

fn create_nosync_folder(base_path: &PathBuf, nosync_path: &PathBuf) -> Result<()> {
    if base_path.exists() {
        if nosync_path.exists() {
            println!("{} already exists", nosync_path.display());
        } else {
            println!(
                "Moving {} to {}",
                base_path.display(),
                nosync_path.display()
            );
            fs::rename(base_path, nosync_path)?;
        }
    } else {
        println!("Creating {}", nosync_path.display());
        fs::create_dir_all(nosync_path)?;
    }
    Ok(())
}

fn create_symlink(base_path: &PathBuf, nosync_path: &PathBuf) -> Result<()> {
    println!("Creating symlink");
    if base_path.exists() {
        fs::remove_dir_all(base_path)?;
    }
    unix::fs::symlink(nosync_path, base_path)?;
    if nosync_path.to_str().unwrap().contains("node_modules") {
        install_nodemodules()?;
    }
    Ok(())
}

fn install_nodemodules() -> Result<()> {
    let choices = vec!["yarn", "npm", "cnpm", "Do not install"];
    let install_choice = Select::new("Choose installation method:", choices).prompt()?;

    let command = match install_choice {
        "yarn" => Some("yarn"),
        "npm" => Some("npm install"),
        "cnpm" => Some("cnpm install"),
        _ => None,
    };

    if let Some(cmd) = command {
        let output = ProcessCommand::new("sh").arg("-c").arg(cmd).status()?;

        if !output.success() {
            eprintln!("Command execution failed");
        };
    };

    Ok(())
}

fn add_gitignore(pwd: &PathBuf, folder: &str) -> Result<()> {
    let gitignore_path = pwd.join(".gitignore");
    let mut gitignore_content = String::new();

    if gitignore_path.exists() {
        gitignore_content = fs::read_to_string(&gitignore_path)?;
    }

    if !gitignore_content.contains(folder) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(gitignore_path)?;

        writeln!(file, "\n# no-sync")?;
        writeln!(file, "{}*", folder)?;
        println!("Added {} to .gitignore", folder);
    } else {
        println!("{} already in .gitignore", folder);
    }

    Ok(())
}
