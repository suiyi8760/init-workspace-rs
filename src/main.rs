use std::path::Path;
use std::process::Command;
use std::{env, fs};
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
        println!(
            "❌ Project folder '{}' already exists, exiting.",
            project_name
        );
        return;
    }

    // Create project folder
    fs::create_dir(project_name).expect("❌ Failed to create project folder");

    // Create project template
    fs::create_dir(project_name.to_owned() + "/creatives")
        .expect("❌ Failed to create creatives folder");
    fs::create_dir(project_name.to_owned() + "/prod-doc")
        .expect("❌ Failed to create prod-doc folder");

    println!("✅ Project folders created");

    if args.init {
        let new_dir = format!("./{}/{}", project_name, project_name);

        // 创建
        fs::create_dir(&new_dir).expect("❌ Failed to create prod-doc folder");

        match env::set_current_dir(&new_dir) {
            Ok(()) => println!("✅ Successfully changed working directory to {}", new_dir),
            Err(e) => println!("❌ Failed to change working directory: {}", e),
        }

        let mut child = Command::new("pnpm")
            .arg("create")
            .arg("@yy/hago-app")
            .spawn()
            .unwrap();

        // 不await一下主进程结束 子进程也结束了 就看不到spawn的输出内容
        match child.try_wait() {
            Ok(Some(status)) => println!("exited with: {status}"),
            Ok(None) => {
                println!("⌛️ status not ready yet, let's really wait");
                let res = child.wait();
                println!("result: {res:?}");
            }
            Err(e) => println!("❌ error attempting to wait: {e}"),
        }
    }

    // Git clone
    if let Some(git) = args.git {
        let git_command = format!("git clone {} ./{}/{}", git, project_name, project_name);
        let mut git_output = Command::new("sh")
            .arg("-c")
            .arg(&git_command)
            .spawn()
            .expect("Failed to run git clone");

        // 不await一下主进程结束 子进程也结束了 就看不到spawn的输出内容
        match git_output.try_wait() {
            Ok(Some(status)) => println!("finish git clone: {status}"),
            Ok(None) => {
                println!("⌛️ git cloning... :{}", &git_command);
                let res = git_output.wait();
                println!("result: {res:?}");
            }
            Err(e) => println!("❌ error attempting to wait: {e}"),
        }
    } else {
        println!("✅ Skipping git clone as git repository is not specified.");
    }
}
