use tracing_subscriber::EnvFilter;

pub fn init(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("monstock_cli=debug")
    } else {
        EnvFilter::new("monstock_cli=info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}
