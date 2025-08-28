use clap::{Parser, Subcommand};

// main.rs using the library crate
use task_mgr::Status;
// use task_mgr::core::domain::Task; // this is impossible since only `Status` is re-exported from the library
use task_mgr::actions::create::create;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    action: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create {
        #[arg(short, long, help = "The label of the task")]
        label: String,
        #[arg(short, long, help = "The description of the task")]
        desc: String,
        #[arg(short, long, help = "The priority of the task")]
        priority: u8,
    },
    List,
    Delete {
        id: String,
    },
    Edit {
        id: String,
        #[arg(short, long, help = "The label of the task")]
        label: String,
        #[arg(short, long, help = "The description of the task")]
        desc: String,
        #[arg(short, long, help = "The priority of the task")]
        priority: u8,
        #[arg(short, long, help = "The status of the task")]
        status: Status,
    },
}

fn main() {
    let args = Cli::parse();

    match args.action {
        Commands::Create {
            label,
            desc,
            priority,
        } => {
            create(label, desc, priority);
        }
        Commands::List => {
            println!("Listing all tasks");
            // Here you would add code to list all tasks from your storage
        }
        Commands::Delete { id } => {
            println!("Deleting task with ID: {}", id);
            // Here you would add code to delete the task from your storage
        }
        Commands::Edit {
            id,
            label,
            desc,
            priority,
            status,
        } => {
            println!(
                "Editing task with ID: {} to {} - {} with priority {} and status {:?}",
                id, label, desc, priority, status
            );
            // Here you would add code to edit the task in your storage
        }
    }
}
