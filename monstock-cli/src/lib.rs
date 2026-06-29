pub mod cli;
pub mod config;
pub mod process;
pub mod seed;
pub mod doctor;
pub mod logging;

use std::path::Path;
use std::process::{Child, Command, Stdio};
use clap::Parser;
use cli::{MonstockCli, DevTarget, BuildTarget};
use config::MonstockConfig;
use tracing::info;

pub fn run() {
    let cli = MonstockCli::parse();

    logging::init(cli.verbose);

    let config_path = cli
        .config
        .clone()
        .or_else(default_config_path);
    let config = load_config(config_path.as_deref());

    match &cli.command {
        cli::Commands::Dev { target } => cmd_dev(&config, target),
        cli::Commands::Build { target, release } => cmd_build(&config, target, *release),
        cli::Commands::Seed { file, dry_run } => cmd_seed(&config, file.as_deref(), *dry_run),
        cli::Commands::Test => cmd_test(&config),
        cli::Commands::Lint => cmd_lint(&config),
        cli::Commands::Clean => cmd_clean(&config),
        cli::Commands::Doctor => cmd_doctor(),
        cli::Commands::Completions { shell } => cmd_completions(*shell),
    }
}

fn default_config_path() -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    let local = cwd.join("monstock.toml");
    if local.exists() {
        return Some(local.to_string_lossy().to_string());
    }
    if let Some(config_dir) = dirs::config_dir() {
        let global = config_dir.join("monstock").join("cli.toml");
        if global.exists() {
            return Some(global.to_string_lossy().to_string());
        }
    }
    None
}

fn load_config(path: Option<&str>) -> MonstockConfig {
    match path {
        Some(p) => match config::load_from_file(p) {
            Ok(c) => {
                tracing::trace!("Loaded config from {}", p);
                c
            }
            Err(e) => {
                tracing::warn!("Failed to load config from {}: {}", p, e);
                info!("Using default config");
                MonstockConfig::default()
            }
        },
        None => {
            tracing::trace!("Using default config");
            MonstockConfig::default()
        }
    }
}

fn spawn_dev_process(cmd: &mut Command, name: &str) -> Child {
    info!("Starting {} dev server...", name);
    match cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn() {
        Ok(child) => child,
        Err(e) => {
            tracing::error!("Failed to start {}: {}", name, e);
            std::process::exit(1);
        }
    }
}

fn run_dev_tauri(config: &MonstockConfig) {
    let tauri_dir = project_root().join("monstock-tauri");
    let mut cmd = Command::new("cargo");
    cmd.args(["tauri", "dev"])
        .current_dir(&tauri_dir);
    if config.tauri.webkit_disable_dmabuf {
        cmd.env("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    let mut child = spawn_dev_process(&mut cmd, "Tauri");
    let status = child.wait();
    match status {
        Ok(s) if s.success() => info!("Tauri dev stopped"),
        Ok(s) => tracing::error!("Tauri dev exited with: {}", s),
        Err(e) => tracing::error!("Failed to wait for Tauri dev: {}", e),
    }
}

fn run_dev_egui() {
    let root = project_root();
    let mut cmd = Command::new("cargo");
    cmd.args([
        "watch", "-c", "-x", "run --bin monstock-desktop",
        "-w", "monstock-desktop/src/",
        "-w", "monstock-core/src/",
    ]).current_dir(&root);
    let mut child = spawn_dev_process(&mut cmd, "egui");
    let status = child.wait();
    match status {
        Ok(s) if s.success() => info!("egui dev stopped"),
        Ok(s) => tracing::error!("egui dev exited with: {}", s),
        Err(e) => tracing::error!("Failed to wait for egui dev: {}", e),
    }
}

fn run_both() {
    let root = project_root();
    let tauri_dir = root.join("monstock-tauri");

    let mut tauri_cmd = Command::new("cargo");
    tauri_cmd
        .args(["tauri", "dev"])
        .current_dir(&tauri_dir)
        .env("WEBKIT_DISABLE_DMABUF_RENDERER", "1")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut egui_cmd = Command::new("cargo");
    egui_cmd
        .args(["watch", "-c", "-x", "run --bin monstock-desktop",
            "-w", "monstock-desktop/src/",
            "-w", "monstock-core/src/"])
        .current_dir(&root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut tauri_child = match tauri_cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to start Tauri: {}", e);
            return;
        }
    };

    let mut egui_child = match egui_cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to start egui: {}", e);
            kill_child(&tauri_child);
            return;
        }
    };

    info!("Both dev servers started. Press Ctrl+C to stop.");
    let result = wait_for_any(&mut tauri_child, &mut egui_child);

    kill_child(&tauri_child);
    kill_child(&egui_child);

    if let Err(e) = result {
        tracing::error!("Dev process error: {}", e);
    }
}

fn kill_child(child: &Child) {
    let _ = std::process::Command::new("kill")
        .arg(child.id().to_string())
        .status();
}

fn wait_for_any(a: &mut Child, b: &mut Child) -> Result<(), std::io::Error> {
    loop {
        if a.try_wait()?.is_some() {
            return Ok(());
        }
        if b.try_wait()?.is_some() {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn project_root() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("monstock-cli should have a parent directory")
        .to_path_buf()
}

fn cmd_dev(_config: &MonstockConfig, target: &DevTarget) {
    match target {
        DevTarget::Tauri => run_dev_tauri(_config),
        DevTarget::Egui => run_dev_egui(),
        DevTarget::All => run_both(),
    }
}

fn run_build_tauri(release: bool) {
    let tauri_dir = project_root().join("monstock-tauri");
    let runner = process::Runner::new(false);

    if !release {
        info!("Building Tauri frontend (dev mode)...");
        let mut cmd = Command::new("pnpm");
        cmd.args(["run", "build"]).current_dir(&tauri_dir);
        let mut child = match cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to start pnpm build: {}", e);
                std::process::exit(1);
            }
        };
        let status = child.wait();
        if let Ok(s) = status {
            if !s.success() {
                tracing::error!("pnpm build failed");
                std::process::exit(s.code().unwrap_or(1));
            }
        }
    }

    info!("Building Tauri backend...");
    let mut args = vec!["tauri", "build"];
    if release {
        args.push("--release");
    }
    runner.run_inherited("cargo", &args, Some(&tauri_dir)).ok();
}

fn run_build_egui(release: bool) {
    let root = project_root();
    let runner = process::Runner::new(false);

    info!("Building egui desktop...");
    let mut args = vec!["build", "--bin", "monstock-desktop"];
    if release {
        args.push("--release");
    }
    runner.run_inherited("cargo", &args, Some(&root)).ok();
}

fn run_build_all(release: bool) {
    run_build_egui(release);
    run_build_tauri(release);
}

fn cmd_build(_config: &MonstockConfig, target: &BuildTarget, release: bool) {
    match target {
        BuildTarget::Tauri => run_build_tauri(release),
        BuildTarget::Egui => run_build_egui(release),
        BuildTarget::All => run_build_all(release),
    }
}

fn cmd_seed(config: &MonstockConfig, file: Option<&str>, dry_run: bool) {
    if dry_run {
        info!("[DRY RUN] Would seed database at: {}", config.database.path);
        if let Some(f) = file {
            info!("[DRY RUN] Would load seed data from: {}", f);
        }
        info!("[DRY RUN] Plan: 500 products, 10 suppliers, 30 days of expenses/sales/POs");
        info!("[DRY RUN] Run without --dry-run to apply");
        return;
    }

    if let Some(f) = file {
        info!("Loading seed data from: {}", f);
        seed::run_from_file(&config.database.path, f);
    } else {
        seed::run(&config.database.path);
    }
}

fn cmd_test(_config: &MonstockConfig) {
    let root = project_root();
    let runner = process::Runner::new(false);
    let tauri_dir = root.join("monstock-tauri");

    info!("Running Rust tests...");
    runner.run_inherited("cargo", &["test", "--workspace"], Some(&root)).ok();

    info!("Running frontend tests...");
    runner.run_inherited("pnpm", &["test"], Some(&tauri_dir)).ok();
}

fn cmd_lint(_config: &MonstockConfig) {
    let root = project_root();
    let runner = process::Runner::new(false);

    info!("Running clippy...");
    runner.run_inherited("cargo", &["clippy", "--workspace", "--", "-D", "warnings"], Some(&root)).ok();

    info!("Checking formatting...");
    runner.run_inherited("cargo", &["fmt", "--check"], Some(&root)).ok();
}

fn cmd_clean(_config: &MonstockConfig) {
    let root = project_root();
    let runner = process::Runner::new(false);
    let tauri_dir = root.join("monstock-tauri");
    let tauri_node_modules = tauri_dir.join("node_modules");
    let tauri_dist = tauri_dir.join("dist");

    info!("Cleaning cargo target...");
    runner.run_inherited("cargo", &["clean"], Some(&root)).ok();

    info!("Cleaning node_modules...");
    if tauri_node_modules.exists() {
        std::fs::remove_dir_all(&tauri_node_modules).ok();
        info!("  removed node_modules");
    }

    info!("Cleaning frontend dist...");
    if tauri_dist.exists() {
        std::fs::remove_dir_all(&tauri_dist).ok();
        info!("  removed dist");
    }

    info!("Clean complete");
}

fn cmd_doctor() {
    doctor::run();
}

fn cmd_completions(shell: clap_complete::Shell) {
    use clap::CommandFactory;
    let mut cmd = MonstockCli::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
}
