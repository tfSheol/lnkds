mod voyager;

use clap::{AppSettings, Clap};
use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;

use voyager::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Teddy F. <pro@teddyfontaine.fr>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// "li_at" from LinkedIn Cookie
    li_at: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(setting = AppSettings::ColoredHelp)]
    Profile(Profile),
}

/// A subcommand to Get LinkedIn profile (basic informations, experiences, certificates, ...)
#[derive(Clap)]
struct Profile {
    /// User account you whant to parse
    user: String,
    #[clap(short, long)]
    output: Option<String>,
}

async fn get_profile(profile: voyager::Profile, output: Option<String>) {
    // let bar = ProgressBar::new(1000);
    let profile_result: serde_json::Value = profile.request().await.unwrap();
    println!("result {}", profile_result.get("included").unwrap());

    let result: Vec<_> = profile_result.get("included").unwrap().as_array().unwrap().iter().filter(|x| x.get("employmentTypeUrn").is_some()).collect();
    // // bar.inc(1);
    // // bar.finish();
    match output {
        Some(path) => {
            let mut w = File::create(format!("./out/{}", path)).unwrap();
            writeln!(&mut w, "{}", format!("{}", json!(result))).unwrap();
        }
        None => println!("no path"),
    }
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    match opts.subcmd {
        SubCommand::Profile(t) => {
            if t.user == "" {
                return;
            }
            get_profile(
                voyager::Profile {
                    li_at: opts.li_at,
                    user_identity: t.user,
                },
                t.output,
            )
            .await;
        }
    }
}
