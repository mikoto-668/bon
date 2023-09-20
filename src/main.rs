mod bon_lib;

use crate::bon_lib::{
    config::{edit::edit, init::init},
    template::{add::add, delete::delete, edit::edit_template, gen::gen, update::update},
    voyage::voyage,
};
use clap::{ArgMatches, Command};

fn main() {
    let matches: ArgMatches = Command::new("bon")
        .about("The command that prepare some files for your directory")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .version("1.0.1")
        .subcommand(
            Command::new("config")
                .about("Configuration for bon")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(Command::new("init").about("Initialize your config"))
                .subcommand(Command::new("edit").about("Edit user information")),
        )
        .subcommand(
            Command::new("template")
                .about("Template for bon")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(Command::new("add").about("Add template profile"))
                .subcommand(Command::new("update").about("Update template profile"))
                .subcommand(Command::new("delete").about("Delete template profile"))
                .subcommand(Command::new("gen").about("Generate template from profile"))
                .subcommand(Command::new("edit").about("Edit user template")),
        )
        .subcommand(Command::new("voyage").about("Prepare files from your config"))
        .get_matches();

    match matches.subcommand() {
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("init", _sub_matches)) => init(),
            Some(("edit", _sub_matches)) => edit(),
            _ => unreachable!(
                "Exhausted list of subcommands and subcommand_required prevents `None`"
            ),
        },
        Some(("template", sub_matches)) => match sub_matches.subcommand() {
            Some(("add", _sub_matches)) => add(),
            Some(("update", _sub_matches)) => update(),
            Some(("delete", _sub_matches)) => delete(),
            Some(("gen", _sub_matches)) => gen(),
            Some(("edit", _sub_matches)) => edit_template(),
            _ => unreachable!(
                "Exhausted list of subcommands and subcommand_required prevents `None`"
            ),
        },
        Some(("voyage", _sub_matches)) => voyage(),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
