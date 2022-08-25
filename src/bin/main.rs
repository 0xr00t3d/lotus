use lottas::start;
use std::path::Path;
use clap::{Arg, ArgMatches, Command};
use console::Emoji;

fn args() -> ArgMatches {
    Command::new("Lottas")
        .version("0.1.0-beta")
        .author("Khaled Nassar <knassar702@gmail.com>")
        .about(format!("{} Fast Web Security Scanner written in Rust with Lua Support to make DAST process Faster {}{}",Emoji("⚡", "").to_string(),Emoji("🦀",""),Emoji("🌖","")).as_str())
        .subcommands(
            vec![
            Command::new("tool")
            .about("RUN A TOOL")
            .arg(
                Arg::new("input-type")
                .help("input type")
                .takes_value(true)
                
            ),Command::new("scan")
            .about(format!("{} Run a Scanning Module/s on your target",Emoji("🎯","")).as_str())
            .arg(
                Arg::new("report")
                .help("Report file")
                .long("report")
                .takes_value(true)
                .validator(|report| {
                    if Path::new(report).exists() {
                        Err(format!("{} exist", report))
                    } else {
                        Ok(())
                    }
                })
                )
            .arg(
                Arg::new("modules")
                    .help("The modules to use")
                    .long("modules")
                    .validator(|module| {
                        if module.contains(" ") {
                            Err("Modules must be separated by a space")
                        } else {
                            Ok(())
                        }
                    })
                    .possible_values(&["xss"])
                    .default_values(&["xss"])
                    .takes_value(true),
            )
            .arg(
                Arg::new("delay")
                    .help("The delay between requests")
                    .long("delay")
                    .validator(|delay| {
                        if delay.parse::<u64>().is_err() {
                            Err("Delay must be a number")
                        } else {
                            Ok(())
                        }
                    })
                    .takes_value(true),
            )
            .arg(
                Arg::new("data")
                    .help("The data to send")
                    .long("data")
                    .short('d')
                    .default_value("")
                    .takes_value(true),
            )
            .arg(
                Arg::new("headers")
                    .help("The headers to send")
                    .long("headers")
                    .short('H')
                    .default_value("")
                    .multiple_occurrences(true)
                    .takes_value(true),
            )
            .arg(
                Arg::new("method")
                    .help("The HTTP method to use")
                    .long("method")
                    .takes_value(true)
                    .default_value("GET"),
            )
            .arg(
                Arg::new("concurrency")
                    .help("The number of concurrent requests to make (default: 10)")
                    .long("concurrency")
                    .short('c')
                    .default_value("10")
                    .validator(|s| {
                        if s.parse::<usize>().is_ok() {
                            Ok(s.parse::<usize>().unwrap())
                        } else {
                            Err("Concurrency must be a number".to_string())
                        }
                    })
                    .takes_value(true),
            )
            .arg(
                Arg::new("file")
                    .help("The file containing the URLs to scan")
                    .long("file")
                    .validator(|s| {
                        if std::path::Path::new(s).exists() {
                            Ok(())
                        } else {
                            Err("File does not exist".to_string())
                        }
                    })
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::new("redirect")
                    .help("The Number of redirects to follow")
                    .long("redirect")
                    .short('r')
                    .validator(|s| {
                        if s.parse::<u8>().is_ok() {
                            Ok(())
                        } else {
                            Err("Redirects must be a number".to_string())
                        }
                    })
                    .default_value("0")
                    .takes_value(true),
            )
            .arg(
                Arg::new("proxy")
                    .help("The proxy to use")
                    .long("proxy")
                    .short('p')
                    .default_value("")
                    .takes_value(true),
            )
            .arg(
                Arg::new("timeout")
                    .help("The timeout in seconds")
                    .long("timeout")
                    .short('t')
                    .takes_value(true)
                    .validator(|s| {
                        if s.parse::<u64>().is_ok() {
                            Ok(s.parse::<u64>().unwrap())
                        } else {
                            Err("Timeout must be a number".to_string())
                        }
                    })
                    .default_value("30"),
            )])
        .get_matches()
}

fn main() {
    args();
    start();
}
