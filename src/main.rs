mod bon_lib;

use crate::bon_lib::config::{
    init::init,
    edit::edit
};
use clap::{Arg, ArgAction, ArgMatches, Command};

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
                .subcommand(
                    Command::new("edit")
                        .about("Edit user information")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("username")
                                .help("your name")
                                .short('u')
                                .long("user")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("mail")
                                .help("your mail address")
                                .short('m')
                                .long("mail")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("editor")
                                .help("your editor")
                                .short('e')
                                .long("editor")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(Command::new("add").about("Add file profile"))
                .subcommand(Command::new("update").about("Update file profile"))
                .subcommand(Command::new("delete").about("Delete file profile")),
        )
        .subcommand(Command::new("voyage").about("Prepare files from your config"))
        .get_matches();

    match matches.subcommand() {
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("init", _sub_matches)) => init(),
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
