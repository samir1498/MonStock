use std::process::Command;

struct Check {
    name: &'static str,
    description: &'static str,
    program: &'static str,
    args: &'static [&'static str],
}

const CHECKS: &[Check] = &[
    Check { name: "cargo", description: "Rust build tool", program: "cargo", args: &["--version"] },
    Check { name: "rustc", description: "Rust compiler", program: "rustc", args: &["--version"] },
    Check { name: "node", description: "Node.js runtime", program: "node", args: &["--version"] },
    Check { name: "pnpm", description: "Package manager", program: "pnpm", args: &["--version"] },
    Check { name: "cargo-watch", description: "Watch mode (cargo-watch)", program: "cargo", args: &["watch", "--version"] },
    Check { name: "cargo-tauri", description: "Tauri CLI", program: "cargo", args: &["tauri", "--version"] },
];

pub fn run() {
    println!("MonStock Doctor");
    println!("================\n");

    let mut all_ok = true;

    for check in CHECKS {
        let status = run_check(check);
        all_ok &= status;
    }

    println!();
    if all_ok {
        println!("All checks passed.");
    } else {
        println!("Some checks failed. Install missing tools and try again.");
    }
}

fn run_check(check: &Check) -> bool {
    let output = Command::new(check.program)
        .args(check.args)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("?")
                .to_string();
            println!("  ✓  {:<14} {} ({})", check.name, check.description, version);
            true
        }
        _ => {
            println!("  ✗  {:<14} {} — not found", check.name, check.description);
            false
        }
    }
}
