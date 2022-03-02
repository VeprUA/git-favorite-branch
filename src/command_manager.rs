use std::{collections::HashMap, process::exit};

use crate::{config::Config, git_helpers::switch_branch};

pub struct CommandManager {
    pub config: Config,
}

impl CommandManager {
    pub fn add_branch(&mut self, key: String, branch_name: String) {
        self.config.state.insert(key, branch_name);
        self.config.save()
    }

    pub fn switch_to_branch(&self, key: String) {
        match self.config.state.get(&key) {
            Some(branch_name) => switch_branch(branch_name).unwrap(),
            None => {
                println!("No branch name set for key {}", key);
                exit(1)
            }
        };
    }

    pub fn delete_branch(&mut self, key: String) {
        self.config.state.remove(&key).unwrap();
        self.config.save()
    }

    pub fn list_branches(&self) {
        let ref state = self.config.state;
        state
            .into_iter()
            .for_each(|(k, v)| println!("{} => {}", k, v))
    }

    pub fn clear_branches(&mut self) {
        self.config.state = HashMap::new();
        self.config.save()
    }

    pub fn print_branch_name(&self, key: String) {
        let branch_name = self.config.state.get(&key).unwrap();
        println!("{}", branch_name);
    }
}