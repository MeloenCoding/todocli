use clap:: {
  Args,
  Parser,
  Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Add, mark as Done or Show todolist
    List(ListCommand),
    /// Edit config
    Conf(ConfigCommand)
}
// List

    #[derive(Debug, Args)]
    pub struct ListCommand {
        #[clap(subcommand)]
        pub command: ListSubCommand,
    }

    #[derive(Debug, Subcommand)]
    pub enum ListSubCommand{
        /// Add item to todo list
        Add(AddTodo),
        /// Mark as Done
        Done(TodoDone),
        /// Show all items in todo list
        Show,
    }
    #[derive(Debug, Args)]
    pub struct AddTodo{
        /// The thing you need to get done
        pub todo_item: String,
    }

    #[derive(Debug, Args)]
    pub struct TodoDone{
        /// The index of the item you want to remove
        pub index_of_item: i32,
        /// Remove todo out of the List
        #[arg(short)]
        pub remove: bool,
    }

// Config

    #[derive(Debug, Args)]
    pub struct ConfigCommand {
        #[clap(subcommand)]
        pub command: ConfigSubCommand
    }

    #[derive(Debug, Subcommand)]
    pub enum ConfigSubCommand {
        /// remote config settings
        Remote(RemLocationEntity),
        /// local config settings (make sure you put '[]' in your own .json file)
        Local(LocLocationEntity)
    }

    #[derive(Debug, Args)]
    pub struct RemLocationEntity{
        /// link to your api (example: "https://your.cool/api")
        #[arg(short)]
        pub location: Option<String>,
        /// enable the remote config
        #[arg(short)]
        pub enable: bool
    }

    #[derive(Debug, Args)]
    pub struct LocLocationEntity{
        /// location of the file (example: "D:/Coding/todocli/data.json")
        #[arg(short)]
        pub location: Option<String>,
        /// enable the remote config
        #[arg(short)]
        pub enable: bool
    }


