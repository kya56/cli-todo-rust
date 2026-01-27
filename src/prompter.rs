use dialoguer::{Confirm, Input, Select};

pub trait Prompter {
    fn select(&self, items: &[String], prompt: &str) -> Result<Option<usize>, String>;

    fn input(&self, prompt: &str, initial: &str) -> Result<Option<String>, String>;

    fn confirm(&self, prompt: &str) -> Result<bool, String>;
}

pub struct DialoguerPrompter;

impl Prompter for DialoguerPrompter {
    fn select(&self, items: &[String], prompt: &str) -> Result<Option<usize>, String> {
        Select::new()
            .with_prompt(prompt)
            .items(items)
            .interact()
            .map(Some)
            .map_err(|e| e.to_string())
    }

    fn input(&self, prompt: &str, initial: &str) -> Result<Option<String>, String> {
        Input::<String>::new()
            .with_prompt(prompt)
            .with_initial_text(initial)
            .interact_text()
            .map(Some)
            .map_err(|e| e.to_string())
    }

    fn confirm(&self, prompt: &str) -> Result<bool, String> {
        Confirm::new()
            .with_prompt(prompt)
            .default(false)
            .interact()
            .map_err(|e| e.to_string())
    }
}
