use clap::Parser;
use kitlib::starter::{StarterConfig, start_session};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "kstarter",
    version,
    about = "Create a detached kitwin desktop session and exit"
)]
struct Args {
    /// Persistent session name to create.
    #[arg(long)]
    session: String,

    /// Virtual display width in pixels.
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Virtual display height in pixels.
    #[arg(long, default_value_t = 1200)]
    height: u32,

    /// Preferred Xvfb display number.
    #[arg(long, default_value_t = 99)]
    display: u8,

    /// Optional command to launch inside the virtual display after JWM starts.
    #[arg(long)]
    exec: Option<String>,

    /// Path to a JWM rc file.
    #[arg(long)]
    jwm_config: Option<PathBuf>,

    /// Kill and replace an existing live session with the same name.
    #[arg(long)]
    replace: bool,
}

pub fn run() {
    let args = Args::parse();
    let name = args.session.clone();

    match start_session(StarterConfig {
        name: args.session,
        display: args.display,
        width: args.width,
        height: args.height,
        jwm_config: args.jwm_config,
        exec: args.exec,
        replace: args.replace,
    }) {
        Ok(state) => {
            println!(
                "kstarter: started session '{}' on DISPLAY=:{} ({}x{})",
                name, state.display, state.width, state.height
            );
        }
        Err(err) => {
            eprintln!("kstarter: {}", err);
            std::process::exit(1);
        }
    }
}
