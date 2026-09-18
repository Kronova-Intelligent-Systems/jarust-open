use jarust_analyzer::RepositoryScanner;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from(".")
    };

    println!("Scanning target: {}", target_dir.display());
    let scanner = RepositoryScanner::new();

    match scanner.scan_path(&target_dir) {
        Ok(report) => {
            println!(
                "Migration Urgency Score: {} / 100\n",
                report.calculate_migration_score()
            );
            for (cat, count) in &report.hotspot_counts {
                println!(
                    "[*] {} (Occurrences: {}) -> Use {}",
                    cat.label(),
                    count,
                    cat.rust_remedy()
                );
            }
        }
        Err(err) => eprintln!("Scan error: {}", err),
    }
}
