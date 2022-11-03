mod args;

use args::{ConfigSubCommand, EntityType, ListSubCommand, TodoArgs};
use clap::Parser;
use colored::Colorize;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    data: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    data: String,
    completed: bool,
}
#[derive(Deserialize, Debug)]
struct Config {
    local: bool,
    local_location: String,
    remote_location: String,
    remote_key: String,
    config_dir: String,
    appId: String,
    appKey: String
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // build config mutable
    let mut config: Config = Config {
        local: true,
        local_location: "".to_string(),
        remote_location: "".to_string(),
        remote_key: "".to_string(),
        config_dir: "".to_string(),
        appId: "".to_string(),
        appKey: "".to_string()
    };
    if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "todocli") {
        // store the path to the config and data
        let config_path: &Path = proj_dirs.config_dir();
        let data_path: &Path = proj_dirs.data_dir();
        
        let config_file: String = fs::read_to_string(proj_dirs.config_dir().join("todocli.toml"))
            .unwrap_or("".to_string());

        if config_file == "" {
            // if config file doesn't exist create the missing directories
            std::fs::create_dir_all(config_path).unwrap();
            std::fs::create_dir_all(data_path).unwrap();

            let byte_string: String = format!("local = true\nlocal_location = {:?}\nremote_location = \"https://your.crazy/api\"\nremote_key = \"\"\nconfig_dir = {:?}\nappId = \"\"\nappKey = \"\"", 
                proj_dirs.data_dir().join("data.json").as_os_str(), 
                proj_dirs.config_dir().join("todocli.toml").as_os_str()
            );

            // create config and data file if it doesn'st exist and write some data to it
            let mut new_config_file: File = File::create(proj_dirs.config_dir().join("todocli.toml"))
                .expect("can't create config file");
            new_config_file
                .write_all(byte_string.as_bytes())
                .expect("can't write config file");

            let mut new_data_file: File = File::create(proj_dirs.data_dir().join("data.json"))
                .expect("can't create data file");
             new_data_file
                .write_all(b"[]")
                .expect("can't write data file");
            println!(
                "        -════╡ {} {}{} ╞════-",
                "todocli".bright_cyan().blink(),
                "v".purple(),
                "0.2.1".yellow()
            );
            println!("Run this command again and enjoy.");
            std::process::exit(0x0100);
        }
        config = toml::from_str(&config_file.to_string()).expect("Error opening config file.");
    }

    let args: TodoArgs = TodoArgs::parse();
    match &args.entity_type {
        EntityType::List(list) => match &list.command {
            ListSubCommand::Add(add_command) => {
                if !config.local {
                    let api_link: String = format!("{}", &config.remote_location);
                    let res: TodoList = reqwest::Client::new()
                        .post(api_link)
                        .header("Content-Type", "application/json")
                        .json(&serde_json::json!({
                            "appId": config.appId.to_string(),
                            "appKey": config.appKey.to_string(),
                            "clientKey": config.remote_key.to_string(),
                            "endpoint": "/add",
                            "data": {
                                "data": add_command.todo_item.to_string(),
                                "completed": false
                            }
                        }))
                        .send()
                        .await?
                        .json()
                        .await?;
                    draw_cli(&res);
                } else {
                    let path: String = config.local_location;
                    let path_to_data: &Path = Path::new(&path);
                    let data_file: String = match fs::read_to_string(path_to_data) {
                        Ok(data_string) => data_string,
                        Err(_error) => { 
                            println!(
                                "        -════╡ {} {}{} ╞════-",
                                "todocli".bright_cyan().blink(),
                                "v".purple(),
                                "0.2.1".yellow()
                            );
                            println!("Can't read/find data file.");
                            println!("If you made a data file and configured it, make sure there is '[]' in the data file so it is valid json.");
                            std::process::exit(0x0100);
                        }
                    };
                    let mut data: Vec<Todo> = serde_json::from_str(&data_file).unwrap();
                    let new_todo: Todo = Todo {
                        data: (add_command.todo_item.to_string()),
                        completed: (false),
                    };
                    data.push(new_todo);
                    std::fs::write(path_to_data, serde_json::to_string_pretty(&data).unwrap())
                        .unwrap();
                    draw_cli(&TodoList { data: (data) });
                }
            }
            ListSubCommand::Done(done_command) => {
                if !config.local {
                    let api_link: String = format!("{}", config.remote_location);
                    let res: TodoList = reqwest::Client::new()
                        .post(api_link)
                        .header("Content-Type", "application/json")
                        .json(&serde_json::json!({
                            "appId": config.appId.to_string(),
                            "appKey": config.appKey.to_string(),
                            "clientKey": config.remote_key.to_string(),
                            "endpoint": "/done",
                            "data": {
                                "index": done_command.index_of_item,
                                "shouldRemove": done_command.remove
                            }
                        }))
                        .send()
                        .await?
                        .json()
                        .await?;

                    draw_cli(&res);
                } else {
                    let path: String = config.local_location;
                    let path_to_data: &Path = Path::new(&path);
                    let data_file: String = match fs::read_to_string(path_to_data) {
                        Ok(data_string) => data_string,
                        Err(_error) => { 
                            println!(
                                "        -════╡ {} {}{} ╞════-",
                                "todocli".bright_cyan().blink(),
                                "v".purple(),
                                "0.2.1".yellow()
                            );
                            println!("Can't read/find data file.");
                            println!("If you made a data file and configured it, make sure there is '[]' in the data file so it is valid json.");
                            std::process::exit(0x0100);
                        }
                    };
                    let mut data: Vec<Todo> = serde_json::from_str(&data_file).unwrap();
                    let index: usize = done_command.index_of_item as usize;
                    if done_command.remove {
                        data.remove(index);
                    } else {
                        data[index].completed = !data[index].completed;
                    }
                    std::fs::write(path_to_data, serde_json::to_string_pretty(&data).unwrap())
                        .unwrap();
                    draw_cli(&TodoList { data: (data) });
                }
            }
            ListSubCommand::Show => {
                if !config.local {
                    let api_link: String = format!("{}", config.remote_location);
                    let res: TodoList = reqwest::Client::new()
                        .post(api_link)
                        .header("Content-Type", "application/json")
                        .json(&serde_json::json!({
                            "appId": config.appId.to_string(),
                            "appKey": config.appKey.to_string(),
                            "clientKey": config.remote_key.to_string(),
                            "endpoint": "/show",
                            "data": {}
                        }))
                        .send()
                        .await?
                        .json()
                        .await?;
                    draw_cli(&res);
                } else {
                    let path: String = config.local_location;
                    let path_to_data: &Path = Path::new(&path);
                    let data_file: String = match fs::read_to_string(path_to_data) {
                        Ok(data_string) => data_string,
                        Err(_error) => { 
                            println!(
                                "        -════╡ {} {}{} ╞════-",
                                "todocli".bright_cyan().blink(),
                                "v".purple(),
                                "0.2.1".yellow()
                            );
                            println!("Can't read/find data file.");
                            println!("If you made a data file and configured it, make sure there is '[]' in the data file so it is valid json.");
                            std::process::exit(0x0100);
                        }
                    };
                    let read_data = serde_json::from_str(&data_file);
                    match read_data {
                        Ok(data) => {
                            draw_cli(&TodoList { data: (data) });
                        }
                        Err(_error) => {
                            println!(
                                "        -════╡ {} {}{} ╞════-",
                                "todocli".bright_cyan().blink(),
                                "v".purple(),
                                "0.2.1".yellow()
                            );
                            println!("Can't find data file. Please configure one.")
                        }
                    }
                }
            }
        },
        EntityType::Conf(option) => {
            match &option.command {
                ConfigSubCommand::Local(loc_command) => {
                    if loc_command.location.is_none() && !loc_command.enable {
                        println!("You need to supply subcommands! \nrun 'todocli conf local -h' for help");
                        std::process::exit(0x0100);
                    }
                    dbg!(&config.config_dir);

                    if loc_command.location.is_some() {
                        let byte_string = format!("local = {}\nlocal_location = {:?}\nremote_location = {:?}\nremote_key = {:?}\nconfig_dir = {:?}\nappId = {:?}\nappKey = {:?}", &config.local, &loc_command.location.as_ref().unwrap(), &config.remote_location, &config.remote_key, &config.config_dir, &config.appId, &config.appKey);
                        let mut new_config_file: File = OpenOptions::new()
                            .write(true)
                            .open(&config.config_dir)
                            .expect("Error opening config file.");
                        new_config_file
                            .write_all(byte_string.as_bytes())
                            .expect("can't write file");

                        if loc_command.enable {
                            let byte_string = format!("local = true\nlocal_location = {:?}\nremote_location = {:?}\nremote_key = {:?}\nconfig_dir = {:?}\nappId = {:?}\nappKey = {:?}", &loc_command.location.as_ref().unwrap(), &config.remote_location, &config.remote_key, &config.config_dir, &config.appId, &config.appKey);
                            let mut new_config_file: File = OpenOptions::new() 
                                .write(true)
                                .open(&config.config_dir)
                                .expect("Error opening config file.");
                            new_config_file
                                .write_all(byte_string.as_bytes())
                                .expect("can't write file");
                        }
                    } else if loc_command.enable {
                        let byte_string = format!("local = true\nlocal_location = {:?}\nremote_location = {:?}\nremote_key = {:?}\nconfig_dir = {:?}\nappId = {:?}\nappKey = {:?}", &config.local_location, &config.remote_location, &config.remote_key, &config.config_dir, &config.appId, &config.appKey);
                        let mut new_config_file: File = OpenOptions::new()
                            .write(true)
                            .open(&config.config_dir)
                            .expect("Error opening config file.");
                        new_config_file
                            .write_all(byte_string.as_bytes())
                            .expect("can't write file");
                    }
                }
                ConfigSubCommand::Remote(loc_command) => {
                    println!("{:?}", &config.config_dir);
                    if loc_command.location.is_none() && !loc_command.enable {
                        println!("You need to supply subcommands! \nrun 'todocli conf remote -h' for help");
                        std::process::exit(0x0100);
                    }

                    if loc_command.location.is_some() {
                        let byte_string = format!("local = {}\nlocal_location = {:?}\nremote_location = {:?}\nremote_key = {:?}\nconfig_dir = {:?}\nappId = {:?}\nappKey = {:?}", &config.local, &config.local_location, &loc_command.location.as_ref().unwrap(), &config.remote_key, &config.config_dir, &config.appId, &config.appKey);
                        let mut new_config_file: File = OpenOptions::new()
                            .write(true)
                            .open(&config.config_dir)
                            .expect("Error opening config file.");
                        println!("{:?}", new_config_file);
                        new_config_file
                            .write(byte_string.as_bytes())
                            .expect("can't write file");

                        if loc_command.enable {
                            let byte_string = format!("local = false\nlocal_location = {:?}\nremote_location = {:?}\nremote_key = {:?}\nconfig_dir = {:?}\nappId = {:?}\nappKey = {:?}", &config.local_location, &loc_command.location.as_ref().unwrap(), &config.remote_key, &config.config_dir, &config.appId, &config.appKey);
                            let mut new_config_file: File = OpenOptions::new()
                                .write(true)
                                .open(&config.config_dir)
                                .expect("Error opening config file.");
                            new_config_file
                                .write(byte_string.as_bytes())
                                .expect("can't write file");
                        }
                    } else if loc_command.enable {
                        let byte_string = format!("local = true\nlocal_location = {:?}\nremote_location = {:?}\nremote_key = {:?}\nconfig_dir = {:?}\nappId = {:?}\nappKey = {:?}", &config.local_location, &config.remote_location, &config.remote_key, &config.config_dir, &config.appId, &config.appKey);
                        let mut new_config_file: File = OpenOptions::new()
                            .write(true)
                            .open(&config.config_dir)
                            .expect("Error opening config file.");
                        new_config_file
                            .write_all(byte_string.as_bytes())
                            .expect("can't write file");
                    }
                }
            }
        }
    }
    Ok(())
}

fn draw_cli(list: &TodoList) {
    println!(
        "╔═════════╡ {} {}{} ╞═══════",
        "todocli".bright_cyan().blink(),
        "v".purple(),
        "0.2.1".yellow()
    );
    println!("║ ");
    let mut i: i32 = 0;
    for todo_item in &list.data {
        if todo_item.completed {
            println!("║  ╭ [{:?}] ", i);
            println!("║  ╰─╴ [{}] {}", "x".green(), todo_item.data);
        } else {
            println!("║  ╭ [{:?}] ", i);
            println!("║  ╰─╴ [ ] {}", todo_item.data);
        }
        println!("║ ");
        i += 1;
    }
    println!("╚══════════════════════════════════");
}
