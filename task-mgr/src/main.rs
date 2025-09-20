use clap::{Parser, Subcommand};

// use task_mgr::core::domain::Task; // this is impossible since only `Status` is re-exported from the library
use anyhow::{Context, Result};
use std::fs::OpenOptions;
use task_mgr::{LocalSaver, Status, TaskManager};
use tracing::error;
use tracing_subscriber::{EnvFilter, Layer, prelude::*};

const LOG_FILE: &str = "task_mgr.log";
const INFO_LOG_LEVEL: &str = "info";
const DEBUG_LOG_LEVEL: &str = "debug";
const TASKS_FILE: &str = "tasks.json";

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

fn init_tracing() {
    let stdout_env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(INFO_LOG_LEVEL));

    let file_env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEBUG_LOG_LEVEL));

    // Stdout layer
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(true)
        .with_filter(stdout_env_filter);
    // File layer
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Failed to open log file");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_target(true)
        .with_filter(file_env_filter);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
}

fn main() -> Result<()> {
    let args = Cli::parse();
    init_tracing();

    let mut task_manager = TaskManager::new(LocalSaver::new(TASKS_FILE))
        .inspect_err(|e| {
            error!("Failed to initialize task manager: {e}");
        })
        .context("Failed to initialize task manager")?;

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
    };
    Ok(())
}
