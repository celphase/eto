use std::{path::Path, process::Command};

use anyhow::{anyhow, Context, Error};
use clap::Args;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use tracing::{event, Level};

pub fn command(command: PatchCommand) -> Result<(), Error> {
    // Scan for a package.etopack
    let result = glob::glob(&command.package);
    let package = if let Some(result) = result
        .ok()
        .and_then(|mut paths| paths.next())
        .and_then(|result| result.ok())
    {
        result
    } else {
        return Err(anyhow!("couldn't find package"));
    };

    // If given, wait for a process to close
    if let Some(pid) = command.wait_for {
        event!(Level::INFO, pid, "waiting for process to close");

        let pid = Pid::from_u32(pid);
        let mut system = System::new();
        system.refresh_process(pid);
        let process = system.process(pid);

        if let Some(process) = process {
            process.wait();
        } else {
            event!(Level::WARN, "process not found");
        }
    }

    // Apply the patch
    let directory = Path::new("./");
    eto::package::patch_directory(&package, directory)?;

    // If given, launch a new process
    if let Some(command) = command.on_complete {
        let _ = Command::new(command)
            .spawn()
            .context("failed to run on_complete")?;
    }

    Ok(())
}

/// Patch the working directory by finding and applying a package.
///
/// This command is intended to be used either from a script, or called by another process to
/// update itself.
///
/// If you are using this for self-updating, make sure you copy the eto binary to a temporary
/// location so it can update itself too.
/// Use the `--wait-for` flag to wait until the original process closes, and `--on_complete` to
/// restart it.
#[derive(Args, Debug)]
pub struct PatchCommand {
    /// Location of the package to apply. Allows glob patterns (for example, `*.etopack`).
    #[arg(short, long)]
    package: String,

    /// If given, wait for a process with this process identifier to close before applying.
    #[arg(long)]
    wait_for: Option<u32>,

    /// If given, run this command after completion.
    #[arg(long)]
    on_complete: Option<String>,
}
