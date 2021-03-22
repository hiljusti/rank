use crate::sigi::actions::{Action, Command};
use clap::{App, Arg, ArgMatches, SubCommand};

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version (0.1.6) of sigi.
pub const SIGI_VERSION: &str = "0.1.6";

fn create_aliases<'a>() -> Vec<&'a str> {
    vec!["push", "add", "do", "start", "new"]
}

fn complete_aliases<'a>() -> Vec<&'a str> {
    vec!["done", "finish", "fulfill"]
}
fn delete_aliases<'a>() -> Vec<&'a str> {
    vec!["pop", "remove", "cancel", "drop", "abandon", "retire"]
}
fn delete_all_aliases<'a>() -> Vec<&'a str> {
    vec![
        "purge",
        "pop-all",
        "remove-all",
        "cancel-all",
        "drop-all",
        "abandon-all",
        "retire-all",
    ]
}
fn list_aliases<'a>() -> Vec<&'a str> {
    vec!["show"]
}
fn list_all_aliases<'a>() -> Vec<&'a str> {
    vec!["all"]
}
fn length_aliases<'a>() -> Vec<&'a str> {
    vec!["count", "size"]
}
fn is_empty_aliases<'a>() -> Vec<&'a str> {
    vec!["empty"]
}
fn next_aliases<'a>() -> Vec<&'a str> {
    vec!["later", "punt", "bury"]
}
fn swap_aliases<'a>() -> Vec<&'a str> {
    vec![]
}
fn rot_aliases<'a>() -> Vec<&'a str> {
    vec!["rotate"]
}

pub fn get_action() -> Action {
    let create_aliases = create_aliases();
    let complete_aliases = complete_aliases();
    let delete_aliases = delete_aliases();
    let delete_all_aliases = delete_all_aliases();
    let list_aliases = list_aliases();
    let list_all_aliases = list_all_aliases();
    let length_aliases = length_aliases();
    let is_empty_aliases = is_empty_aliases();
    let next_aliases = next_aliases();
    let swap_aliases = swap_aliases();
    let rot_aliases = rot_aliases();

    let matches: ArgMatches = App::new("sigi")
        .version(SIGI_VERSION)
        .about("An organizational tool.")
        .arg(
            Arg::with_name("topic")
                .short("t")
                .long("topic")
                .value_name("TOPIC")
                .help("Manage items in a specific topic")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Omit any leading labels or symbols. Recommended for use in shell scripts")
        )
        // TODO: Collapse repetition
        .subcommands(vec![
            SubCommand::with_name("create")
                .about(&*usage_message(&create_aliases, "Creates a new item"))
                .aliases(&create_aliases)
                .arg(
                    Arg::with_name("name")
                        .value_name("NAME")
                        .required(true)
                        .multiple(true),
                ),
            SubCommand::with_name("peek")
                .about(&*usage_message(&next_aliases, "Peek at the current item. (This is the default behavior if no command is given)"))
                .aliases(&next_aliases),
            SubCommand::with_name("complete")
                .about(&*usage_message(&complete_aliases, "Marks the current item as successfully completed"))
                .aliases(&complete_aliases),
            SubCommand::with_name("delete")
                .about(&*usage_message(&delete_aliases, "Removes the current item"))
                .aliases(&delete_aliases),
            SubCommand::with_name("delete-all")
                .about(&*usage_message(&delete_all_aliases, "Removes all items"))
                .aliases(&delete_all_aliases),
            SubCommand::with_name("list")
                .about(&*usage_message(&list_aliases, "Lists the current priority items"))
                .aliases(&list_aliases),
            SubCommand::with_name("list-all")
                .about(&*usage_message(&list_all_aliases, "Lists all items"))
                .aliases(&list_all_aliases),
            SubCommand::with_name("length")
                .about(&*usage_message(&length_aliases, "Gives the total count of all items"))
                .aliases(&length_aliases),
            SubCommand::with_name("is-empty")
                .about(&*usage_message(&is_empty_aliases, "Tells you if the stack is empty"))
                .aliases(&is_empty_aliases),
            SubCommand::with_name("next")
                .about(&*usage_message(&next_aliases, "Moves the next item to current, and moves current to last."))
                .aliases(&next_aliases),
            SubCommand::with_name("swap")
                .about(&*usage_message(&swap_aliases, "Swaps the two most current items."))
                .aliases(&swap_aliases),
            SubCommand::with_name("rot")
                .about(&*usage_message(&rot_aliases, "Rotates the three most current items."))
                .aliases(&rot_aliases),
            // TODO: Need an idea of "organize" or "re-order"
            // TODO: Need support for stack-of-stack management
        ])
        .get_matches();

    let to_command = |name: &str| matches.subcommand_matches(name);
    let command_is = |name: &str| to_command(name).is_some();

    let command: Command = if let Some(matches) = to_command("create") {
        if let Some(name_bits) = matches.values_of("name") {
            let name = name_bits.collect::<Vec<_>>().join(" ");
            Command::Create(name)
        } else {
            error_no_command("create")
        }
    } else if command_is("complete") {
        Command::Complete
    } else if command_is("delete") {
        Command::Delete
    } else if command_is("delete-all") {
        Command::DeleteAll
    } else if command_is("list") {
        Command::List
    } else if command_is("all") {
        Command::ListAll
    } else if command_is("length") {
        Command::Length
    } else if command_is("is-empty") {
        Command::IsEmpty
    } else if command_is("next") {
        Command::Next
    } else if command_is("swap") {
        Command::Swap
    } else if command_is("rot") {
        Command::Rot
    } else {
        Command::Peek
    };

    let topic = matches.value_of("topic").unwrap_or("sigi").to_owned();
    let quiet = matches.is_present("quiet");

    Action {
        command,
        topic,
        quiet,
    }
}

fn usage_message(aliases: &[&str], text: &str) -> String {
    if aliases.is_empty() {
        String::from(text)
    } else {
        format!("Or: [{}]\n{}", aliases.join(", "), text)
    }
}

fn error_no_command(name: &str) -> Command {
    eprintln!("Error, not enough arguments given for: {}", name);
    Command::Peek
}
