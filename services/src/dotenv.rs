use std::{collections::HashMap, env::current_dir, fs::File, io::{BufRead, BufReader}};

/// Represents a Dotenv parser, holding environment variables in a HashMap.
pub struct Dotenv {
    env_variable: HashMap<String, String>
}

impl Dotenv {
    /// Creates a new `Dotenv` instance by reading and parsing a .env file.
    ///
    /// It looks for a .env file in the current working directory.
    /// Returns `Some(Self)` if successful, `None` otherwise (e.g., file not found, permission issues).
    pub fn new() -> Option<Self> {
        
        // Attempt to create a buffered reader for the .env file.
        let file_buffer = {

            // Get the current working directory.
            let current_dir = current_dir();

            // If getting the current directory fails, return None.
            if current_dir.is_err() {
                return None;
            }

            // Unwrap the current directory path.
            let current_dir = current_dir.unwrap();

            // Try to open the .env file located in the current directory.
            let get_env_file = File::open(format!("{}/.env", current_dir.to_str().unwrap()));

            // If opening the file fails, return None.
            if get_env_file.is_err() {
                return None;
            }

            // Unwrap the opened file.
            let env_file = get_env_file.unwrap();
            // Create a buffered reader for efficient line-by-line reading.
            let read_file = BufReader::new(env_file);

            // Return the buffered reader wrapped in Some.
            Some(read_file)

        };

        // If the file buffer could not be created, return None.
        if file_buffer.is_none() {
            return None;
        }

        // Unwrap the file buffer.
        let file_buffer = file_buffer.unwrap();

        // Initialize a new HashMap to store environment variables.
        let mut env_var = HashMap::new();

        // Iterate over each line in the buffered file reader.
        for line in file_buffer.lines() {

            // Skip lines that cause an error during reading.
            if line.is_err() {
                continue;
            }

            // Unwrap the successfully read line.
            let unwrap_line = line.unwrap();
            // Trim leading/trailing whitespace from the line.
            let parse_text = unwrap_line.trim();

            // Skip empty lines or lines that start with a '# ' (comments).
            if parse_text.is_empty() || parse_text.starts_with("#") {
                continue;
            }

            // Process lines that contain an '=' sign, indicating a key-value pair.
            if parse_text.contains("=") {

                // Split the line into a key and a value at the first '='.
                // Panics if splitting fails, which should not happen if `contains("=")` is true.
                let (key, value) = parse_text.split_once("=").expect("Failed to split variable");

                // Convert the key to a String and trim any whitespace.
                let make_key = String::from(key).trim().to_string();
                // Convert the value to a String and trim any whitespace.
                let mut make_value = String::from(value).trim().to_string();


                // Remove leading double quotes or single quotes from the value.
                if make_value.starts_with("\"") || make_value.starts_with("'") {
                    make_value.remove(0);
                }

                // Remove trailing double quotes or single quotes from the value.
                if make_value.ends_with("\"") || make_value.ends_with("'") {
                    make_value.remove(make_value.len() - 1);
                }

                // Insert the parsed key-value pair into the HashMap.
                env_var.insert(make_key, make_value);
            }

        }

        // Return a new Dotenv instance with the populated HashMap.
        Some( 
            Self {
                env_variable: env_var
            }
        )
    }


    /// Reads the value associated with a given key from the loaded environment variables.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the name of the environment variable.
    ///
    /// # Panics
    ///
    /// Panics if the key is not found in the environment variables.
    ///
    /// # Returns
    ///
    /// A `String` containing the value of the environment variable.
    pub fn read_value(&self, key: &str) -> String {

        // Attempt to get the key-value pair for the given key.
        let value = self.env_variable.get_key_value(key);

        // If the key is not found, panic.
        if value.is_none() {
            panic!("Failed to read value on key {key}");
        }

        // Unwrap the found key-value pair.
        let parse_value = value.unwrap();

        // Return the value part (second element of the tuple) as a String.
        parse_value.1.to_string()
    } 
}