use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        [] | ["--help"] | ["-h"] => {
            println!("diskwhy about [--json]");
            println!("Scaffold only: scanning and cleanup are not implemented.");
            ExitCode::SUCCESS
        }
        ["about"] => {
            let info = disk_explainer_core::capabilities();
            println!("diskwhy {} ({})", info.version, info.stage);
            println!("Storage: {}", info.storage_engine);
            println!("Implemented: about. Scanning and cleanup are not implemented.");
            ExitCode::SUCCESS
        }
        ["about", "--json"] => {
            let result = serde_json::to_string_pretty(&disk_explainer_core::capabilities());
            match result {
                Ok(json) => {
                    println!("{json}");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("Cannot serialize capabilities: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!("Unsupported command. Run diskwhy --help. No files were changed.");
            ExitCode::FAILURE
        }
    }
}
