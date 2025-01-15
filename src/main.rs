/*
CLI app to convert whitespace field separators in command pipelines
into a single comma separator.
*/

// TODO:: make a CLI with clap, and let the delimiter be an argument
//  defaulting to comma

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for line in linedance::input()? {
        let line = line?;
        let new_line = apply(&line);
        println!("{}", new_line);
    }
    Ok(())
}

fn apply(line: &str) -> String {
    shlex::split(line)
        .unwrap_or(vec!["".to_owned()])
        .iter()
        .map(|item| {
            if item.contains(char::is_whitespace) {
                format!("\"{}\"", item)
            } else {
                item.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        assert_eq!(apply("a b c"), "a,b,c");
    }

    #[test]
    fn test_spaces() {
        assert_eq!(apply("a      b    c"), "a,b,c");
    }

    #[test]
    fn test_quotes() {
        assert_eq!(apply(r#"a "b b" c"#), r#"a,"b b",c"#);
    }
}
