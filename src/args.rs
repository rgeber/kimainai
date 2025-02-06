use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::util::args::{default_config_path, expand_tilde};

/// KimaiNai is an anti Kimai API client minimizing the need to interact with time tracking.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short='c', long, default_value = default_config_path().into_os_string(), value_parser=expand_tilde)]
    pub config_file: PathBuf,

    #[command(subcommand)]
    pub command: Option<CliCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommands {
    /// List all customers
    ListCustomers {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },


    ListProjects {},
}