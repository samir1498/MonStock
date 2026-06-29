use std::path::Path;
use std::process::{Command, Output, ExitStatus};
use tracing::{error, info};

#[derive(Debug)]
pub struct Runner {
    dry_run: bool,
}

impl Runner {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub fn run(
        &self,
        program: &str,
        args: &[&str],
        working_dir: Option<&Path>,
    ) -> Result<Output, std::io::Error> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        info!("Running: {} {}", program, args.join(" "));

        if self.dry_run {
            return Ok(Output {
                status: ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });
        }

        let output = cmd.spawn()?.wait_with_output()?;

        if output.status.success() {
            if !output.stdout.is_empty() {
                info!("{}", String::from_utf8_lossy(&output.stdout));
            }
        } else {
            if !output.stderr.is_empty() {
                error!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }

        Ok(output)
    }

    pub fn run_inherited(
        &self,
        program: &str,
        args: &[&str],
        working_dir: Option<&Path>,
    ) -> Result<(), std::io::Error> {
        let mut cmd = Command::new(program);
        cmd.args(args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        info!("Running: {} {}", program, args.join(" "));

        if self.dry_run {
            return Ok(());
        }

        let status = cmd.spawn()?.wait()?;

        if !status.success() {
            error!("Command failed with exit code: {:?}", status.code());
            std::process::exit(status.code().unwrap_or(1));
        }

        Ok(())
    }
}
