extern crate structopt;
#[macro_use]
extern crate cfg_if;

mod cpu_hasher;
#[cfg(feature = "opencl")]
mod gpu_hasher;
#[cfg(feature = "opencl")]
mod ocl;
mod plotter;
mod poc_hashing;
mod scheduler;
mod shabal256;
mod utils;
mod writer;
mod buffer;
mod options;

use crate::plotter::{Plotter, PlotterTask};
use crate::utils::set_low_prio;
use crate::options::Opt;
use crate::utils::free_disk_space;
use structopt::StructOpt;
// use clap::AppSettings::{ArgRequiredElseHelp, DeriveDisplayOrder, VersionlessSubcommands};
// #[cfg(feature = "opencl")]
// use clap::ArgGroup;
// use clap::{App, Arg};
use std::cmp::min;

pub const NONCE_SIZE: u64 = 262144; // Size of nonce in bytes

fn main() {
    let opts = Opt::from_args();

    if opts.low_priority {
        set_low_prio();
    }

    if cfg!(feature = "opencl") && opts.display_opencl {
        ocl::platform_info();
        return;
    }

    // work out number of cpu threads to use
    let cores = sys_info::cpu_num().unwrap() as u8;
    let cpu_threads = if let Some(threads) = opts.threads {
        min(2 * cores, threads)
    } else {
        cores
    };

    // special case: dont use cpu if only a gpu is defined
    #[cfg(feature = "opencl")]
    let cpu_threads = if opts.gpu.is_some() && opts.threads.is_none() {
        0u8
    } else {
        cpu_threads
    };

    let gpus: Option<Vec<String>> = if let Some(gpus) = opts.gpu {
        let gpus: Vec<String> = gpus.split(",")
            .map(|g| g.to_owned())
            .collect();

        for gpu in gpus.iter() {
            if gpu.matches(":").count() != 2 {
                eprintln!("Error: GPU information not specified properly, use -g <platform>:<device_id>:<cores>. To view gpu information run engraver -o.");
                std::process::exit(1);
            }
        }

        Some(gpus)
    } else {
        None
    };

    if opts.plot_size.is_some() && opts.nonces.is_some() {
        eprintln!("Error: Can't specify plot size AND total nonces at the same time; they are both dependant on each other.");
        std::process::exit(1);
    }

    let free_disk_space = free_disk_space(opts.path.to_str().unwrap());

    let nonces = if let Some(size) = opts.plot_size {
        size/NONCE_SIZE
    } else {
        if let Some(nonces) = opts.nonces {
            nonces
        } else {
            // use all avaiblable disk space if nonce parameter has been omitted
            free_disk_space/NONCE_SIZE
        }
    };

    if !opts.quiet {
        println!("Final End Nonce: {}", opts.start_nonce + nonces);
    }

    let p = Plotter::new();
    p.run(
        PlotterTask {
            numeric_id: opts.numeric_id,
            start_nonce: opts.start_nonce,
            nonces,
            output_path: opts.path.display().to_string(),
            mem: opts.memory,
            cpu_threads,
            gpus,
            direct_io: !opts.disable_direct_io,
            async_io: !opts.disable_async_io,
            quiet: opts.quiet,
            benchmark: opts.bench,
            zcb: opts.zero_copy_buffers,
            is_last: false,
        },
        opts.file_num,
    );
}