use clap::Parser;

/// OxyDAV is a Rust WebDAV stress test application designed for the use with ownCloud and nextCloud.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// WebDAV Host (e.g. https://cloud.company.tld)
    #[arg(short = 'H', long)]
    pub host: String,

    /// WebDAV username
    #[arg(short = 'U', long)]
    pub username: String,

    /// WebDAV user password
    #[arg(short = 'P', long)]
    pub password: String,

    /// Remote Sync directory
    #[arg(short, long, default_value = "/")]
    pub remote_sync_path: String,

    /// Number of simultaneous tasks
    #[arg(short='T', long, default_value_t = 1)]
    pub task_count: i32,

    /// Task loop interval (s)
    #[arg(short='I', long, default_value_t = 10)]
    pub client_interval: i32,

    /// Interval fuzziness minimum (ms)
    #[arg(long, default_value_t = 0)]
    pub interval_fuzziness_min: i32,

    /// Interval fuzziness maximum (ms)
    #[arg(long, default_value_t = 3000)]
    pub interval_fuzziness_max: i32,

    /// Minimum amount of skipped downloads in the stress test function (PROPFIND only)
    #[arg(long, default_value_t = 3)]
    pub min_download_skip: i32,

    /// Maximum amount of skipped downloads in the stress test function (PROPFIND only)
    #[arg(long, default_value_t = 10)]
    pub max_download_skip: i32,

    /// Set if you use shibboleth. WebDAV Paths are different.
    #[arg(short='p', long)]
    pub custom_path: Option<String>,

    /// Set WebDAV recursiveness. Available 0,1 and -1 for infnity
    #[arg(short='d', long, default_value_t=1, allow_hyphen_values = true)]
    pub remote_recursion_depth: i32,

    /// Enable file uploads (randomly generated files will be uploaded to the cloud)
    #[arg(short='u', long)]
    pub enable_uploads: bool,

    /// Remote path to which to upload files (must exist)
    #[arg(long, default_value = "/")]
    pub upload_file_path: String,

    /// Upload freq. Minimum (upload a file at least every n requests)
    #[arg(long, default_value_t = 20)]
    pub upload_freq_min: i32,

    /// Upload freq. Maximum (upload a file no more then every n requests)
    #[arg(long, default_value_t = 10)]
    pub upload_freq_max: i32,

    /// Minimum number of files to upload when performing an upload
    #[arg(long, default_value_t = 1)]
    pub upload_count_min: i32,

    /// Maxumum number of files to upload when performing an upload
    #[arg(long, default_value_t = 3)]
    pub upload_count_max: i32,
}