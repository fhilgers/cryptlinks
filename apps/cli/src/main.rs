use cl_core::*;
use cl_qrcode::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Qrcode {
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,

        #[arg(short, long)]
        link: Option<String>,

        #[arg(short, long)]
        password: String,
    },
    Filter {
        #[arg(short, long)]
        link: Option<String>,

        #[arg(short, long)]
        password: String,
    },
}

fn main() {
    let mut text_in = String::new();
    let mut lines = std::io::stdin().lines();

    while let Some(line) = lines.next() {
        let input = line.unwrap();

        text_in.push_str(&input);
        text_in.push('\n');
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Qrcode {
            output,
            link,
            password,
        } => {
            let payload = create_encoded_payload(password, &text_in);

            let payload = if let Some(prefix) = link {
                format!("{}{}", prefix, payload)
            } else {
                payload
            };

            match output {
                Some(path) => {
                    save_svg(&payload, path);
                }
                None => {
                    println!("{}", as_str(&payload));
                }
            }
        }
        Commands::Filter { link: _, password: _ } => todo!(),
    }
}
