use clap::{Parser, Subcommand};

// main.rs using the library crate
use task_mgr::Status;
// use task_mgr::core::domain::Task; // this is impossible since only `Status` is re-exported from the library
use task_mgr::TaskManager;

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
        #[arg(short, long, help = "The ID of the task to edit")]
        target_id: String,
        #[arg(short, long, help = "The new label of the task")]
        label: String,
        #[arg(short, long, help = "The new description of the task")]
        desc: String,
        #[arg(short, long, help = "The new priority of the task")]
        priority: u8,
        #[arg(short, long, help = "The new status of the task")]
        status: Status,
    },
}

fn main() {
    let args = Cli::parse();
    let mut task_manager = TaskManager::new().unwrap_or_else(|err| {
        eprintln!("Error initializing task manager: {}", err);
        std::process::exit(1);
    });

    match args.action {
        Commands::Create {
            label,
            desc,
            priority,
        } => {
            task_manager.create_task(label, desc, priority);
        }
        Commands::List => {
            task_manager.list_tasks();
        }
        Commands::Delete { id } => {
            task_manager.delete_task(&id);
        }
        Commands::Edit {
            target_id,
            label,
            desc,
            priority,
            status,
        } => {
            task_manager.edit_task(&target_id, label, desc, priority, status);
        }
    }
}
