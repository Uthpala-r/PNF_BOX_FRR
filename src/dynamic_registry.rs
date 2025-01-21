use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::cliconfig::CliContext;
use crate::clock_settings::Clock;
use crate::execute::Command;

lazy_static! {
    static ref DYNAMIC_COMMANDS: RwLock<HashMap<&'static str, Command>> = RwLock::new(HashMap::new());
}

pub fn register_command(
    name: &'static str,
    description: &'static str,
    suggestions: Option<Vec<&'static str>>,
    suggestions1: Option<Vec<&'static str>>,
    options: Option<Vec<&'static str>>,
    execute: fn(&[&str], &mut CliContext, &mut Option<Clock>) -> Result<(), String>,
) -> Result<(), String> {
    let command = Command {
        name,
        description,
        suggestions,
        suggestions1,
        options,
        execute,
    };

    let mut commands = DYNAMIC_COMMANDS
        .write()
        .map_err(|_| "Failed to acquire write lock")?;
    
    commands.insert(name, command);
    Ok(())
}

pub fn get_registered_commands() -> Result<HashMap<&'static str, Command>, String> {
    let commands = DYNAMIC_COMMANDS
        .read()
        .map_err(|_| "Failed to acquire read lock")?;
    
    Ok(commands.clone())
}

fn str_to_static(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

fn vec_str_to_static(v: Vec<&str>) -> Vec<&'static str> {
    v.into_iter()
        .map(str_to_static)
        .collect()
}

pub fn convert_to_command(dy_command: Command) -> Command {
    Command {
        name: dy_command.name,
        description: dy_command.description,
        suggestions: dy_command.suggestions,
        suggestions1: dy_command.suggestions1,
        options: dy_command.options,
        execute: dy_command.execute,
    }
}