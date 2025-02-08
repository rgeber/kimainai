use std::path::PathBuf;
use chrono::Weekday;
use clap::{Args, Parser, Subcommand};
use crate::util::args::{default_config_path, expand_tilde};

/// KimaiNai is an anti Kimai API client minimizing the need to interact with time tracking.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    #[arg(short='c', long, default_value = default_config_path().into_os_string(), value_parser=expand_tilde)]
    pub config_file: PathBuf,

    #[command(subcommand)]
    pub command: Option<CliCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommands {
    /// Fetch information for your user
    GetMe {},
    /// List all customers
    ListCustomers {
        // Todo: Delete
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// List all Projects
    ListProjects {},
    /// List all Activities
    ListActivities {
        /// Limit output to a specific project ID
        #[arg(short='p', long)]
        project_id: Option<u32>,
    },
    /// File a single workday. Default values are taken from config file
    FileWorkday(FileWorkdayArgs),

    /// File a period of workdays in bulk. Define a start and an end date as well as days off.
    BulkFileWorkdays(BulkFileWorkdaysArgs),
}

#[derive(Args, Debug, Clone)]
pub struct FileWorkdayArgs {

    /// Activity ID - query with list-activities
    #[arg(short='a', long)]
    pub activity_id: Option<u32>,

    /// Project ID - query with list-projects
    #[arg(short='p', long)]
    pub project_id: Option<u32>,

    /// Description text
    #[arg(short='c', long)]
    pub description: Option<String>,

    /// Duration hours
    #[arg(long)]
    pub duration_hours: Option<u32>,

    /// Duration minutes
    #[arg(long)]
    pub duration_minutes: Option<u32>,

    /// Start time year
    #[arg(short='Y', long)]
    pub start_time_year: Option<u32>,

    /// Start time month
    #[arg(short='M', long)]
    pub start_time_month: Option<u32>,

    /// Start time day
    #[arg(short='d', long)]
    pub start_time_day: Option<u32>,

    /// Start time month
    #[arg(short='H', long)]
    pub start_time_hour: Option<u32>,

    /// Start time hour
    #[arg(short='i', long)]
    pub start_time_minute: Option<u32>,

    /// Break time duration in minutes
    #[arg(short='b', long)]
    pub break_duration: Option<u32>,

    /// Break time start hour
    #[arg(long)]
    pub break_start_hour: Option<u32>,

    /// Break time start minute
    #[arg(long)]
    pub break_start_minute: Option<u32>,

}

#[derive(Args, Debug, Clone)]
pub struct BulkFileWorkdaysArgs {

    /// First day year
    #[arg(short='y', long)]
    pub first_day_year: i32,

    /// First day month
    #[arg(short='m', long)]
    pub first_day_month: u32,

    /// First day ... day
    #[arg(short='d', long)]
    pub first_day_day: u32,

    /// Last day year
    #[arg(short='Y', long)]
    pub last_day_year: i32,

    /// Last day month
    #[arg(short='M', long)]
    pub last_day_month: u32,

    /// Last day ... day
    #[arg(short='D', long)]
    pub last_day_day: u32,

    #[arg(short='e', long, value_delimiter = ',')]
    pub exclude_days: Vec<Weekday>,

    /// Activity ID - query with list-activities
    #[arg(short='a', long)]
    pub activity_id: Option<u32>,

    /// Project ID - query with list-projects
    #[arg(short='p', long)]
    pub project_id: Option<u32>,

    /// Description text
    #[arg(short='c', long)]
    pub description: Option<String>,

    /// Duration hours
    #[arg(long)]
    pub duration_hours: Option<u32>,

    /// Duration minutes
    #[arg(long)]
    pub duration_minutes: Option<u32>,

    /// Start time month
    #[arg(short='H', long)]
    pub start_time_hour: Option<u32>,

    /// Start time hour
    #[arg(short='i', long)]
    pub start_time_minute: Option<u32>,

    /// Break time duration in minutes
    #[arg(short='b', long)]
    pub break_duration: Option<u32>,

    /// Break time start hour
    #[arg(long)]
    pub break_start_hour: Option<u32>,

    /// Break time start minute
    #[arg(long)]
    pub break_start_minute: Option<u32>,

}