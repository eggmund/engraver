use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "Engraver", about = "Engraver - a PoC2 plotter written in Rust")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// Your numeric Account ID
    #[structopt(short = "i", long = "id")]
    pub numeric_id: u64,

    /// Number of nonce where you want to start plotting
    #[structopt(short = "s", long = "sn")]
    pub start_nonce: u64,

    /// How many nonces you want to plot. 0 nonces means to fill disk
    #[structopt(short)]
    pub nonces: Option<u64>,

    /// Total size in bytes that you want to generate.
    #[structopt(short = "u", long = "size")]
    pub plot_size: Option<u64>,

    /// Number of plot files to generate (splits usage between)
    #[structopt(short = "f", long = "file-num", default_value = "1")]
    pub file_num: u64,

    /// Target path for plotfile (optional)
    #[structopt(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Maximum memory usage (optional)
    #[structopt(short, long = "mem", default_value = "0B")]
    pub memory: String,

    /// Maximum cpu threads you want to use (optional)
    #[structopt(short = "c", long = "cpu")]
    pub threads: Option<u8>,

    /// GPU(s) you want to use for plotting (optional). Seperate multiple GPUS using commas, without spaces
    #[cfg(feature = "opencl")]
    #[structopt(name = "platform:device_id:cores", long = "gpu", short = "g")]
    pub gpu: Option<String>,


    /// Disables direct i/o
    #[structopt(short, long = "ddio")]
    pub disable_direct_io: bool,

    /// Disables async writing (single RAM buffer mode)
    #[structopt(short = "a", long = "daio")]
    pub disable_async_io: bool,

    /// Runs engraver with low priority
    #[structopt(short, long = "prio")]
    pub low_priority: bool,

    /// Runs engraver in non-verbose mode
    #[structopt(short, long)]
    pub quiet: bool,

    /// Runs engraver in xPU benchmark mode
    #[structopt(short, long)]
    pub bench: bool,

    /// Display OpenCL platforms and devices
    #[cfg(feature = "opencl")]
    #[structopt(short = "o", long = "opencl")]
    pub display_opencl: bool,

    /// Enables zero copy buffers for shared mem (integrated) gpus
    #[structopt(short, long = "zcb")]
    pub zero_copy_buffers: bool,
}