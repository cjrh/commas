/*
CLI app to convert whitespace field separators in command pipelines
into a single comma separator.
*/

use std::collections::HashMap;
use clap::Parser;
use clap::ArgAction;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[clap(short, long, help = "Format string to use for output")]
    template: Option<String>,

    #[clap(short, long, help = "Delimiter", default_value = ",", help = "Delimiter")]
    delimiter: String,

    #[clap(short, long, action = ArgAction::SetFalse, help = "Lose quotes in output")]
    lose_quotes: bool,

    #[clap(short, long, help = "Character sequence to remove from the ends of each item")]
    strips: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    if let Some(template) = args.template {
        let tmpl = subst::Template::from_str(&template)?;
        for line in linedance::input()? {
            let line = line?;
            // User could include quotes in the template if they wanted.
            let new_line = apply(&line, false, &args.strips);

            let variables = new_line
                .iter()
                .enumerate()
                .map(|(i, item)| ((i + 1).to_string(), item.to_owned()))
                .collect::<HashMap<String, String>>();
            let output = tmpl.expand(&variables)?;
            println!("{}", output);
        }
    } else {
        for line in linedance::input()? {
            let line = line?;
            let new_line = apply(&line, args.lose_quotes, &args.strips);
            println!("{}", new_line.join(&args.delimiter));
        }
    }

    Ok(())
}

fn apply(line: &str, keep_quotes: bool, strips: &Option<String>) -> Vec<String> {
    // Check CLI parameters for the presence of `--fmt` followed by a string
    // If present, use that string as a template with the subst library.
    shlex::split(line)
        .unwrap_or(vec!["".to_owned()])
        .iter()
        .map(|item| {
            if keep_quotes && item.contains(char::is_whitespace) {
                format!("\"{}\"", item)
            } else {
                let item = if let Some(strips) = strips {
                    let c = strips.chars().collect::<Vec<char>>();
                    item.trim_matches(&*c)
                } else {
                    item
                };
                item.to_owned()
            }
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        assert_eq!(apply("a b c", false, &None).join(","), "a,b,c");
    }

    #[test]
    fn test_spaces() {
        assert_eq!(apply("a      b    c", false, &None).join(","), "a,b,c");
    }

    #[test]
    fn test_quotes() {
        assert_eq!(apply(r#"a "b b" c"#, true, &None).join(","), r#"a,"b b",c"#);
    }
}
