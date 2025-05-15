use std::fs;
use std::path::Path;

// Include template files at compile time
const TEMPLATE_CARGO_TOML: &str = include_str!("templates/cargo.toml.template");
const TEMPLATE_LIB_RS: &str = include_str!("templates/lib.rs.template");

fn sanitize_string(name: &str) -> String {
    // Convert to lowercase and replace spaces and dots with hyphens
    let mut sanitized = name
        .to_lowercase()
        .chars()
        .map(|c| match c {
            ' ' | '.' => '-',
            c => c
        })
        .collect::<String>();

    // Replace multiple consecutive hyphens with a single hyphen
    while sanitized.contains("--") {
        sanitized = sanitized.replace("--", "-");
    }

    // Keep only alphanumeric characters and hyphens
    sanitized = sanitized
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();

    // Trim hyphens from start and end
    sanitized = sanitized.trim_matches('-').to_string();

    // If empty after sanitization, return a default name
    if sanitized.is_empty() {
        return "balius-project".to_string();
    }

    sanitized
}

fn create_project(project_name: &str) -> std::io::Result<()> {
    let project_path = Path::new(project_name);

    // Check if directory exists
    if project_path.exists() {
        if project_path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("A file named '{}' already exists", project_name)
            ));
        }

        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Directory '{}' already exists", project_name)
        ));
    }

    // Create project directory and src directory
    fs::create_dir_all(project_path.join("src"))?;

    // Create and write Cargo.toml
    let cargo_content = TEMPLATE_CARGO_TOML.replace("{project_name}", project_name);
    fs::write(project_path.join("Cargo.toml"), cargo_content)?;

    // Create and write src/lib.rs
    fs::write(project_path.join("src/lib.rs"), TEMPLATE_LIB_RS)?;

    Ok(())
}

pub fn execute(args: Vec<String>) {    
    // Process input name from command line arguments
    let input_name = if args.is_empty() {
        eprintln!("Error: Project name is required");
        eprintln!("Usage: cargo balius init <project-name>");
        return;
    } else {
        // Join all arguments with spaces to form the project name
        args.join(" ")
    };

    println!("Initializing Balius project...");

    let project_name = sanitize_string(&input_name);
    
    // If the sanitized name is different from input, show what it was converted to
    if project_name != input_name {
        println!("Package name will be: {}", project_name);
    }

    println!("Preparing project...");
    
    match create_project(&project_name) {
        Ok(_) => println!("Project generated successfully at ./{}", project_name),
        Err(e) => eprintln!("Error generating project: {}", e),
    }
}
