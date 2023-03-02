use std::fs;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "Required: specify project name")]
    project: String,

    #[structopt(short = "g", long = "git", help = "Specify git repository to clone")]
    git: Option<String>,

    #[structopt(short = "i", long = "init", help = "Auto trigger scaffold init")]
    init: bool,
}

fn main() {
    let args = Cli::from_args();
    let project_name = &args.project;

    if Path::new(project_name).exists() {
        println!("Project folder '{}' already exists, exiting.", project_name);
        return;
    }

    // Create project folder
    fs::create_dir(project_name).expect("Failed to create project folder");

    // Create project template
    fs::create_dir(project_name.to_owned() + "/creatives")
        .expect("Failed to create creatives folder");
    fs::create_dir(project_name.to_owned() + "/prod-doc")
        .expect("Failed to create prod-doc folder");

    println!("Created project folders.");

    if args.init {
        // let new_dir = format!("./{}", project_name);

        /* match env::set_current_dir(&new_dir) {
            Ok(()) => println!("Successfully changed working directory to {}", new_dir),
            Err(e) => println!("Failed to change working directory: {}", e),
        } */

        let output = Command::new("cd")
            .arg(format!("./{}", project_name))
            .output()
            .expect("Failed to run git clone");

        println!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
    }

    // Git clone
    if let Some(git) = args.git {
        let git_command = format!("git clone {} ./{}/{}", git, project_name, project_name);
        println!("{}", git_command);
        let git_output = Command::new("sh")
            .arg("-c")
            .arg(git_command)
            .output()
            .expect("Failed to run git clone");

        println!("{}", String::from_utf8_lossy(&git_output.stdout).trim_end());
    } else {
        println!("Skipping git clone as git repository is not specified.");
    }
}
