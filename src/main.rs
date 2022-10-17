mod args;

use args::{EntityType, ListSubCommand, ConfigSubCommand, TodoArgs};
use clap::Parser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    data: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    data: String,
    completed: bool,
    key: String
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = TodoArgs::parse();

    println!("{}", dotenv().expect(".env file not found").display());
    match &args.entity_type {
        EntityType::List(list) => match &list.command {
            ListSubCommand::Add(add_command) => {
                let api_link = format!("{}/add", dotenv::var("API_LINK").unwrap());
                let res = reqwest::Client::new()
                    .post(api_link)
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({
                        "data": add_command.todo_item.to_string(),
                        "completed": false,
                        "key": dotenv::var("KEY").unwrap()
                    }))
                    .send()
                    .await?
                    .json()
                    .await?;

                draw_cli(&res);
            }
            ListSubCommand::Done(done_command) => {
                let api_link = format!("{}/done", dotenv::var("API_LINK").unwrap());
                let res = reqwest::Client::new()
                    .post(api_link)
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({
                        "index": done_command.index_of_item,
                        "key": dotenv::var("KEY").unwrap(),
                        "shouldRemove": done_command.remove
                    }))
                    .send()
                    .await?
                    .json()
                    .await?;

                draw_cli(&res);
            }
            ListSubCommand::Show => {
                let api_link = format!("{}/show", dotenv::var("API_LINK").unwrap());
                let res: TodoList = reqwest::Client::new()
                    .get(api_link)
                    .send()
                    .await?
                    .json()
                    .await?;
                draw_cli(&res);
            }
        },
        EntityType::Config(option) => match &option.command {
            ConfigSubCommand::Location(loc_command) => {
                println!("{}", loc_command.location)
            }
        },
    }
    Ok(())
}

fn draw_cli(list: &TodoList){
    println!("╔═════════╡todocli╞═══════ ");
    println!("║ ");
    let mut i = 0;
    for todo_item in &list.data {
        
        if todo_item.completed {
            println!("║  ╭ [{:?}] ", i);
            println!("║  ╰─╴ [x] {}", todo_item.data );
        }
        else {
            println!("║  ╭ [{:?}] ", i);
            println!("║  ╰─╴ [ ] {}", todo_item.data );
        }
        println!("║ ");
        i += 1;
    }
    println!("╚═════════════════════════");
}