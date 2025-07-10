use std::{collections::HashMap, env::current_dir, fs::File, io::{BufRead, BufReader}};

pub struct Dotenv {
    env_variable: HashMap<String, String>
}

impl Dotenv {
    pub fn new() -> Option<Self> {
        
        let file_buffer = {
            let current_dir = current_dir();

            if current_dir.is_err() {
                return None;
            }

            let current_dir = current_dir.unwrap();

            let get_env_file = File::open(format!("{}/.env", current_dir.to_str().unwrap()));

            if get_env_file.is_err() {
                return None;
            }
            let env_file = get_env_file.unwrap();
            let read_file = BufReader::new(env_file);

            Some(read_file)

        };

        if file_buffer.is_none() {
            return None;
        }

        let file_buffer = file_buffer.unwrap();

        let mut env_var = HashMap::new();

        for line in file_buffer.lines() {
            if line.is_err() {
                continue;
            }
            let unwrap_line = line.unwrap();
            let parse_text = unwrap_line.trim();

            if parse_text.is_empty() || parse_text.starts_with("#") {
                continue;
            }

            if parse_text.contains("=") {
                let (key, value) = parse_text.split_once("=").expect("Failed to split variable");

                let make_key = String::from(key).trim().to_string();
                let mut make_value = String::from(value).trim().to_string();

                if make_value.starts_with("\"") || make_value.starts_with("'") {
                    make_value.remove(0);
                }

                if make_value.ends_with("\"") || make_value.ends_with("'") {
                    make_value.remove(make_value.len() - 1);
                }

                env_var.insert(make_key, make_value);
            }

        }

        Some( 
            Self {
                env_variable: env_var
            }
        )
    }

    pub fn read_value(&self, key: &str) -> String {

        let value = self.env_variable.get_key_value(key);

        if value.is_none() {
            panic!("Failed to read value on key {key}");
        }

        let parse_value = value.unwrap();

        parse_value.1.to_string()
    } 
}