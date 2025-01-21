use crate::cliconfig::CliContext;
use crate::clock_settings::Clock;
use crate::dynamic_registry::register_command;


// Example 1: Simple hello command
pub fn register_custom_commands() -> Result<(), String> {
    register_command(
        "hello",
        "Prints a greeting message",
        Some(vec!["world", "friend"]),
        None,
        None,
        hello_command
    )?;
    Ok(())
}

fn hello_command(args: &[&str], _context: &mut CliContext, _clock: &mut Option<Clock>) -> Result<(), String> {
    match args.get(0).map(|s| *s) {
        Some("world") => println!("Hello, World!"),
        Some("friend") => println!("Hello, Friend!"),
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello there!"),
    }
    Ok(())
}