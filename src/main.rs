mod voyager;

use clap::{AppSettings, Clap};
use indicatif::ProgressBar;
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
}

async fn get_profile(profile: voyager::Profile) {
    let bar = ProgressBar::new(1000);
    println!("ok {}", profile.new());
    let profile_result = profile.request().await.unwrap();
    println!("result {}", profile_result.get("elements").unwrap());
    bar.inc(1);
    bar.finish();
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
            get_profile(voyager::Profile {
                li_at: opts.li_at,
                user_identity: t.user,
            })
            .await;
        }
    }
}
