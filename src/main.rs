mod bon_lib;

use crate::bon_lib::config::{edit::edit, init::init};
use clap::{ArgMatches, Command};

fn main() {
    let matches: ArgMatches = Command::new("bon")
        .about("The command that prepare some files for your directory")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .version("1.0.0")
        .subcommand(
            Command::new("config")
                .about("Configuration for bon")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(Command::new("init").about("Initialize your config"))
                .subcommand(Command::new("edit").about("Edit user information"))
                .subcommand(Command::new("add").about("Add file profile"))
                .subcommand(Command::new("update").about("Update file profile"))
                .subcommand(Command::new("delete").about("Delete file profile")),
        )
        .subcommand(Command::new("voyage").about("Prepare files from your config"))
        .get_matches();

    match matches.subcommand() {
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("init", _sub_matches)) => init().unwrap(),
            Some(("edit", _sub_matches)) => edit(),
            Some(("add", _sub_matches)) => {}
            Some(("update", _sub_matches)) => {}
            Some(("delete", _sub_matches)) => {}
            _ => unreachable!(
                "Exhausted list of subcommands and subcommand_required prevents `None`"
            ),
        },
        Some(("voyage", _sub_matches)) => {}
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
