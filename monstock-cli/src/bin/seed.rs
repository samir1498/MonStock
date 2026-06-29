fn main() {
    let db_path = monstock_cli::config::DatabaseConfig::default().path;
    monstock_cli::seed::run(&db_path);
}
