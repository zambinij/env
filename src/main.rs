use std::{io::{self, BufReader, Read}, fs::{File}, collections::HashSet};

struct Env {
    content: String,
}

impl Env {
    fn new (content: String) -> Env {
        let content = content.clone();

        Env { content }
    }
}

fn main() {
    println!("Please enter the two environment files you want to compare (separated by commas), the first one should be the example.");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let envs: Vec<&str> = user_input.trim().split(',').collect();

    if envs.len() != 2 {
        println!("Please enter two env files names separated by commas");
        return;
    }
    
    let example_env_path: String = envs[0].trim().to_string();
    let example_env_content: Env = read_file(example_env_path);
    let example_keys: Vec<&str> = extract_env_keys(&example_env_content.content);

    let user_env_path: String = envs[1].trim().to_string();
    let user_env_content: Env = read_file(user_env_path);
    let user_keys: Vec<&str> = extract_env_keys(&user_env_content.content);

    let missing_from_user: Vec<&str> = find_missing_keys(&example_keys, &user_keys);

    println!("You are missing the following keys: \n{:#?}", missing_from_user);
}

fn read_file(file_path: String) -> Env {
    // TODO:: remove this in prod
    let dev_file_path: String = format!("./fixtures/{}", file_path);
    
    let example_env_file: File = File::open(dev_file_path).unwrap();
    let mut example_env_buf_reader: BufReader<File> = BufReader::new(example_env_file);
    let mut example_env_content: String = String::new();

    example_env_buf_reader.read_to_string(&mut example_env_content).unwrap();

    Env::new(example_env_content)
}

fn extract_env_keys(contents: &str) -> Vec<&str> {
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    let mut env_keys: Vec<&str> = Vec::new();

    lines.into_iter().for_each(|line: &str| {
        let parts: Vec<&str> = line.split('=').collect();
        let env_key: &str = parts[0].trim();

        env_keys.push(env_key);
    });

    env_keys
}

fn find_missing_keys<'a>(example_keys: &'a Vec<&str>, user_keys: &'a Vec<&str>) -> Vec<&'a str> {
    let example_keys_set: HashSet<_> = example_keys.iter().cloned().collect();
    let user_keys_set: HashSet<_> = user_keys.iter().cloned().collect();

    let missing_from_user: Vec<&'a str> = example_keys_set.difference(&user_keys_set).cloned().collect();

    missing_from_user
}
