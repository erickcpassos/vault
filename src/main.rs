use std::env;
use std::fs;
use std::io::Read;
use std::process;
use std::io::Write;

fn get_file_contents(file_path: &str) -> String {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut content = String::new();

    if let Err(e) = file.read_to_string(&mut content) {
        eprintln!("Couldn't read contents from file: {}", e);
    };

    content
}

fn list_credentials() {
    

    let content = get_file_contents("data.txt");

    // careful with \n or \r\n
    let lines= content.split("\n").collect::<Vec<&str>>();

    println!("Saved credentials:");
    for i in lines {
        if let Some(credential_name) = i.split_ascii_whitespace().next() {
            println!("{}", credential_name);
        }
    }
}

fn add_credential(params: &AddParams) {
    println!("Adding credentials");

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("data.txt")
        .unwrap();

    let AddParams {name, username, password} = params;

    if let Err(e) = writeln!(file, "{name} {username} {password}") {
        eprintln!("Couldn't write to file: {}", e);
    }

}

fn get_credential(params: &GetParams) {

    let content = get_file_contents("data.txt");

    // careful with \n or \r\n
    let lines= content.split("\n").collect::<Vec<&str>>(); 

    let mut found_credential = false;

    for i in lines {
        let data = i.split_ascii_whitespace().collect::<Vec<&str>>();
        if data.len() > 1 {
            let credential_name = data[0];

            if credential_name == params.0 {

                let credential_username;
                let credential_password;

                if data.len() == 2 {
                    credential_password = data[1];
                    credential_username = "None";
                } else {
                    credential_username = data[1];
                    credential_password = data[2];
                }

                found_credential = true;
                println!("Name: {}\nUsername: {}\nPassword: {}", credential_name, credential_username, credential_password);
            }
        }
    }

    if !found_credential { println!("No credentials found for name \"{}\".", params.0); };
}

fn default_action() {
    println!("This is not a valid action.")
}

fn run(config: &Config) {

    match config.params {
        Params::List(_) => { list_credentials(); },
        Params::Add(ref p) => { add_credential(&p); },
        Params::Get(ref p) => { get_credential(&p); },
        Params::Invalid(_) => { default_action(); }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    run(&config);

}

enum Action {
    Get,
    Add,
    List,
    Invalid
}

struct ListParams;
struct GetParams(String);
struct AddParams {
    name: String,
    username: String,
    password: String
}
struct InvalidParams;

enum Params {
    List(ListParams),
    Get(GetParams),
    Add(AddParams),
    Invalid(InvalidParams),
}

struct Config {
    _action: Action,
    params: Params,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        Config {
            _action: Config::get_config_enum(&args),
            params: Config::get_params(&args),
        }
    }

    fn get_config_enum(args: &[String]) -> Action {

        if args.len() <= 1 {
            return Action::Invalid;
        }

        let action = &args[1];
        match action.as_str() {
            "get" => Action::Get,
            "add" => Action::Add,
            "list" => Action::List,
            _ => Action::Invalid
        }
    }

    fn get_params(args: &[String]) -> Params {

        let action = Config::get_config_enum(args);
        let params = &args[2..];

        match action {
            Action::List => Params::List(ListParams),
            Action::Get => {
                if params.len() == 1 {
                    Params::Get(GetParams(params[0].clone()))                    
                } else {
                    println!("vault get requires 1 parameter, but {} parameters were used.", params.len());
                    process::exit(1);
                }
            },
            Action::Add => {
                if params.len() == 2 {
                    Params::Add(
                        AddParams {
                            name: params[0].clone(),
                            password: params[1].clone(),
                            username: String::new() 
                            // maybe make this username None
                        }
                    )               
                } else if params.len() == 3 {
                    Params::Add(
                        AddParams {
                            name: params[0].clone(),
                            username: params[1].clone(), 
                            password: params[2].clone(),
                        }
                    )  
                } else {
                    println!("vault add requires 2 or 3 parameters, but {} parameters were used.", params.len());
                    process::exit(1)
                }
            },
            Action::Invalid => {
                Params::Invalid(InvalidParams)
            }
        } 

    }
}