mod voyager;

use clap::{AppSettings, Clap};
use indicatif::ProgressBar;
use serde::ser::Serialize;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;

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

fn insert_item<T>(
    map: &mut serde_json::Map<std::string::String, serde_json::Value>,
    title: &str,
    value: Option<&T>,
) where
    T: Serialize,
{
    if value.is_some() {
        map.insert(
            title.to_string(),
            serde_json::to_value(value.unwrap()).unwrap(),
        );
    }
}

async fn get_profile(profile: voyager::Profile, output: Option<String>) {
    // let bar = ProgressBar::new(1000);
    let profile_result: serde_json::Value = profile.request().await.unwrap();
    println!("result {}", profile_result.get("included").is_some());

    // let result: Vec<_> = profile_result.get("included").unwrap().as_array().unwrap().iter().filter(|x| x.get("employmentTypeUrn").is_some()).collect();
    let result: Vec<_> = profile_result
        .get("included")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter(|x| {
            x.get("$type")
                .unwrap()
                .as_str()
                .unwrap()
                .eq("com.linkedin.voyager.dash.identity.profile.Position")
        })
        .map(|p| {
            let mut map = serde_json::Map::new();
            insert_item(
                &mut map,
                "starts_at",
                Some(&json!({
                    "month": p.get("dateRange").unwrap().get("start").unwrap().get("month").unwrap(),
                    "year": p.get("dateRange").unwrap().get("start").unwrap().get("year").unwrap()
                })),
            );
            insert_item(
                &mut map,
                "company_linkedin_profile_url",
                Some(&format!("https://fr.linkedin.com/company/{}", "")),
            );
            insert_item(&mut map, "company", p.get("companyName"));
            insert_item(&mut map, "title", p.get("title"));
            insert_item(&mut map, "description", p.get("description"));
            insert_item(&mut map, "location", p.get("locationName"));
            insert_item(&mut map, "logo_url", Some(&format!("{}", "")));
            if p.get("dateRange").unwrap().get("end").is_some() {
                map.insert(
                    "ends_at".to_string(),
                    json!({
                    "month": p.get("dateRange").unwrap().get("end").unwrap().get("month").unwrap(),
                    "year": p.get("dateRange").unwrap().get("end").unwrap().get("year").unwrap()}),
                );
            }
            json!(map)
        })
        .collect();
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
