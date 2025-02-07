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
        // Todo: Delete
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    ListProjects {},
    ListActivities {
        /// Limit output to a specific project ID
        #[arg(short='p', long)]
        project_id: Option<u32>,
    },
    FileWorkDay {
        /// User ID - queried from API if not set.
        #[arg(short='u', long)]
        user_id: Option<u32>,

        /// Customer ID - query with list-customers
        #[arg(short='c', long)]
        customer_id: u32,

        /// Activity ID - query with list-activities
        #[arg(short='a', long)]
        activitiy_id: u32,

        /// Description text
        #[arg(short='D', long)]
        description: Option<String>,

        /// Fixed rate
        #[arg(long, default_value_t=0)]
        fixed_rate: u32,

        /// Hourly rate
        #[arg(long, default_value_t=0)]
        hourly_rate: u32,

        /// Value for exported
        #[arg(long, default_value_t=false)]
        exported: bool,

        /// Value for billable
        #[arg(long, default_value_t=false)]
        billable: bool,

        /// Duration hours
        #[arg(short='H', long)]
        duration_hours: Option<u32>,

        /// Duration minutes
        #[arg(short='i', long)]
        duration_minutes: Option<u32>,

        /// Start time year
        #[arg(short='Y', long)]
        start_time_year: Option<u32>,

        /// Start time month
        #[arg(short='M', long)]
        start_time_month: Option<u32>,

        /// Start time day
        #[arg(short='d', long)]
        start_time_day: Option<u32>,

        /// Break time duration in minutes
        #[arg(short='b', long)]
        break_length: Option<u32>,

        /// Break time start hour
        #[arg(long)]
        break_start_hour: Option<u32>,

        /// Break time start minute
        #[arg(long)]
        break_start_minute: Option<u32>,
    },
}