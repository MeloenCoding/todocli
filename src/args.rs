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
    Config(ConfigCommand)
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
        pub command: ConfigSubCommand,
    }

    #[derive(Debug, Subcommand)]
    pub enum ConfigSubCommand {
        Location(LocationEntity)
    }

    #[derive(Debug, Args)]
    pub struct LocationEntity{
        /// The thing you need to get done
        pub location: String,
    }
