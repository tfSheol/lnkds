mod voyager;

use clap::{crate_version, AppSettings, Clap};
use dotenv::dotenv;
use indicatif::ProgressBar;
use serde::ser::Serialize;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use voyager::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = crate_version!(), author = "Teddy F. <pro@teddyfontaine.fr>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// "li_at" from LinkedIn Cookie
    // #[clap(short, long)]
    li_at: Option<String>,
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
    // println!("result {}", profile_result.get("included").is_some());

    // // let result: Vec<_> = profile_result.get("included").unwrap().as_array().unwrap().iter().filter(|x| x.get("employmentTypeUrn").is_some()).collect();
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

    let profile_result: serde_json::Value = profile.me().await.unwrap();
    println!("result {}", profile_result.get("included").is_some());
    let me_api = profile_result.get("included").unwrap().get(0).unwrap();
    let mut me = serde_json::Map::new();
    insert_item(&mut me, "occupation", me_api.get("occupation"));
    insert_item(&mut me, "firstName", me_api.get("firstName"));
    insert_item(&mut me, "lastName", me_api.get("lastName"));
    insert_item(&mut me, "standardizedPronoun", me_api.get("standardizedPronoun"));
    insert_item(&mut me, "username", me_api.get("publicIdentifier"));
    // insert_item(&mut me, "objectUrn", me_api.get("objectUrn"));
    // insert_item(&mut me, "entityUrn", me_api.get("entityUrn"));
    insert_item(&mut me, "picture", me_api.get("picture"));
    insert_item(&mut me, "trackingId", me_api.get("trackingId"));
    let entity_urn = me_api.get("entityUrn").unwrap().to_string().replace("urn:li:fs_miniProfile:", "").replace("\"", "");

    let experiences_result: serde_json::Value = profile.experiences(entity_urn).await.unwrap();
    println!("result {}", profile_result.get("included").is_some());
    let experiences: Vec<_> = experiences_result
        .get("included")
        .unwrap()
        .get(0)
        .unwrap()
        .get("components")
        .unwrap()
        .get("elements")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            let mut map = serde_json::Map::new();
            println!("{}", p);
            if (p.get("components").is_some()) {
                println!("{}", p.get("components").unwrap());
                let entity = p.get("components").unwrap().get("entityComponent").unwrap();
                // let components = entity.get("subComponents").unwrap().get("components").unwrap().get(0).unwrap().get("components").unwrap().get("components").unwrap().get(0).unwrap();
                // insert_item(&mut map, "company", p.get("companyName"));
                // insert_item(&mut map, "resume", components.get("textComponent").unwrap().get("text").unwrap().get("text"));
                insert_item(&mut map, "company_url", entity.get("image").unwrap().get("actionTarget"));
                insert_item(&mut map, "title", entity.get("title").unwrap().get("text"));
                insert_item(&mut map, "title", entity.get("subtitle").unwrap().get("text"));
                insert_item(&mut map, "time", entity.get("caption").unwrap().get("text"));
                println!("test");
                json!(map)
            } else {
                json!({})
            }
        })
        .collect();

    let result = format!("{}", json!({ "me": me, "experiences": experiences }));
    match output {
        Some(path) => {
            let mut w = File::create(format!("./out/{}.json", path)).unwrap();
            writeln!(&mut w, "{}", result).unwrap();
        }
        None => println!("no path"),
    }
}

fn li_at_value(li_at: Option<String>) -> String {
    match li_at {
        Some(li_at) => li_at,
        None => env::var("LI_AT").expect("$LI_AT is not set"),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

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
                    li_at: li_at_value(opts.li_at),
                    user_identity: t.user,
                },
                t.output,
            )
            .await;
        }
    }
}
