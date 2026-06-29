use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "monstock",
    about = "MonStock development toolkit",
    version,
)]
pub struct MonstockCli {
    #[arg(long, help = "Path to config file")]
    pub config: Option<String>,

    #[arg(long, short, help = "Enable verbose logging")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum DevTarget {
    Tauri,
    Egui,
    All,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum BuildTarget {
    Tauri,
    Egui,
    All,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Start development server with hot reload")]
    Dev {
        #[arg(short, long, value_enum, default_value_t = DevTarget::Tauri, help = "Target frontend [tauri, egui, all]")]
        target: DevTarget,
    },
    #[command(about = "Build the project")]
    Build {
        #[arg(short, long, value_enum, default_value_t = BuildTarget::All, help = "Target frontend [tauri, egui, all]")]
        target: BuildTarget,
        #[arg(long, help = "Build in release mode")]
        release: bool,
    },
    #[command(about = "Seed the database with test data")]
    Seed {
        #[arg(short, long, help = "Path to JSON seed data file")]
        file: Option<String>,
        #[arg(long, help = "Show what would be seeded without modifying the database")]
        dry_run: bool,
    },
    #[command(about = "Run tests")]
    Test,
    #[command(about = "Run linter checks")]
    Lint,
    #[command(about = "Clean build artifacts")]
    Clean,
    #[command(about = "Check development environment")]
    Doctor,
    #[command(about = "Generate shell completion scripts")]
    Completions {
        #[arg(value_enum, help = "Shell type [bash, zsh, fish, powershell]")]
        shell: clap_complete::Shell,
    },
}
