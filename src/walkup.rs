//walkup.rs

use crate::execute::{Mode, Command, get_mode_commands};
use std::collections::HashMap;
use std::fmt;

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct ModeHierarchy {
    pub parent_map: HashMap<Mode, Option<Mode>>,
}

impl ModeHierarchy {
    pub fn new() -> Self {
        let mut parent_map = HashMap::new();
        
        parent_map.insert(Mode::UserMode, None);
        parent_map.insert(Mode::PrivilegedMode, Some(Mode::UserMode));
        parent_map.insert(Mode::ConfigMode, Some(Mode::PrivilegedMode));
        parent_map.insert(Mode::InterfaceMode, Some(Mode::ConfigMode));
        parent_map.insert(Mode::VlanMode, Some(Mode::ConfigMode));
        parent_map.insert(Mode::RouterOSPFMode, Some(Mode::ConfigMode));
        parent_map.insert(Mode::RouterRIPMode, Some(Mode::ConfigMode));
        parent_map.insert(Mode::RouterISISMode, Some(Mode::ConfigMode));  
        parent_map.insert(Mode::RouterEIGRPMode, Some(Mode::ConfigMode));
        parent_map.insert(Mode::RouterBGPMode, Some(Mode::ConfigMode));
        parent_map.insert(Mode::ConfigStdNaclMode("default".to_string()), Some(Mode::ConfigMode));  
        parent_map.insert(Mode::ConfigExtNaclMode("default".to_string()), Some(Mode::ConfigMode));    
        
        Self { parent_map }
    }

    pub fn walkup_find_command(&self, initial_mode: Mode, command: &str) -> Option<Mode> {
        let mut current_mode = initial_mode;
        
        loop {
            // Try to match the command in the current mode
            if Self::is_command_allowed_in_mode(command, &current_mode) {
                return Some(current_mode);
            }
            
            // If no parent mode exists, command is not valid
            let parent_mode = match self.parent_map.get(&current_mode) {
                Some(mode) => mode.clone(),
                None => return None
            };
            
            // If we've reached the top of the hierarchy, stop
            if parent_mode.is_none() {
                return None;
            }
            
            // Move to parent mode
            current_mode = parent_mode.unwrap();
        }
    }

    pub fn is_command_allowed_in_mode(command: &str, mode: &Mode) -> bool {
        match mode {
            Mode::UserMode => 
                command == "enable" ||
                command == "ping" ||
                command == "help" ||
                command == "show" ||
                command == "clear" ||
                command == "reload" ||
                command == "exit",
            Mode::PrivilegedMode => 
                command == "configure" ||
                command == "ping" || 
                command == "exit" || 
                command == "write" ||
                command == "help" ||
                command == "show" ||
                command == "copy" ||
                command == "clock" ||
                command == "clear" ||
                command == "reload" ||
                command == "debug" ||
                command == "undebug" ||
                command == "ifconfig",
            Mode::ConfigMode => 
                command == "hostname" || 
                command == "interface" ||
                command == "ping" ||
                command == "exit" ||
                command == "clear" ||
                command == "tunnel" ||
                command == "access-list" ||
                command == "router" ||
                command == "virtual-template" ||
                command == "help" ||
                command == "write" ||
                command == "vlan" ||
                command == "ip" ||
                command == "service" ||
                command == "set" ||
                command == "enable" ||
                command == "ifconfig" ||  
                command == "ntp" ||
                command == "no" || 
                command == "reload" ||
                command == "crypto",
            Mode::InterfaceMode => 
                command == "shutdown" ||
                command == "no" ||
                command == "exit" ||
                command == "clear" ||
                command == "help" ||
                command == "switchport" ||
                command == "write" ||
                command == "reload" ||
                command == "ip" ,
            Mode::VlanMode => 
                command == "name" ||
                command == "state" ||
                command == "clear" ||
                command == "exit" ||
                command == "help" ||
                command == "reload" ||
                command == "vlan",
            Mode::RouterOSPFMode => 
                command == "network" ||
                command == "neighbor" ||
                command == "exit" ||
                command == "clear" ||
                command == "area" ||
                command == "passive-interface" ||
                command == "distance" ||
                command == "help" ||
                command == "reload" ||
                command == "no" ||
                command == "default-information" ||
                //cmd == "router" ||
                command == "router-id", 
            Mode::RouterRIPMode => 
                command == "network" ||
                command == "passive-interface" ||
                command == "auto-summary" ||
                command == "exit" ||
                command == "clear" ||
                command == "reload" ||
                command == "help" ||
                command == "no",
            Mode::RouterEIGRPMode => 
                command == "network" ||
                command == "passive-interface" ||
                command == "auto-summary" ||
                command == "exit" ||
                command == "clear" ||
                command == "reload" ||
                command == "help" ||
                command == "no" ||
                command == "router-id",
            Mode::RouterISISMode => 
                command == "network" ||
                command == "exit" ||
                command == "clear" ||
                command == "reload" ||
                command == "help" ||
                command == "no" ,
            Mode::RouterBGPMode => 
                command == "network" ||
                command == "exit" ||
                command == "clear" ||
                command == "reload" ||
                command == "help" ||
                command == "no" ||
                command == "distance" || 
                command == "router-id", 
            Mode::ConfigStdNaclMode(_) => 
                command == "deny" ||
                command == "permit" ||
                command == "help" ||
                command == "exit" ||
                command == "clear" ||
                command == "reload" ||
                command == "ip",
            Mode::ConfigExtNaclMode(_) => 
                command == "deny" ||
                command == "permit" ||
                command == "help" ||
                command == "exit" ||
                command == "clear" ||
                command == "reload" ||
                command == "ip",
    
        }
        
    }

}

pub struct CommandContext{
    pub current_mode: Mode,
    pub mode_hierarchy: ModeHierarchy,
    //pub commands: &'a HashMap<&'a str, Command>,
}

impl CommandContext  {
    fn new() -> Self {
        Self {
            current_mode: Mode::UserMode,
            mode_hierarchy: ModeHierarchy::new(),
            //commands,
        }
    }

    fn execute_command(&mut self, command: &str) -> Result<(), String> {
        match self.mode_hierarchy.walkup_find_command(self.current_mode.clone(), command) {
            Some(valid_mode) => {
                if valid_mode != self.current_mode {
                    println!("Walkup: Command '{}' found in {} mode", command, valid_mode);
                }
                self.current_mode = valid_mode;
                self.process_command(command)
            }
            None => Err(format!("Command '{}' not valid in current mode", command)),
        }
    }

    fn process_command(&self, command: &str) -> Result<(), String> {
        println!("Executing command '{}' in {:?} mode", command, self.current_mode);
        Ok(())
    }
}