//! A Wordle result parser

use anyhow::Result;
use regex::Regex;

// A parsed wordle game.
#[derive(Debug, Clone)]
pub struct ParsedWordle<'a> {
    /// the score, out of six for the game
    pub score: &'a str,
    /// the day of the game
    pub day: &'a str,
    /// whether the game was played in hard mode
    pub hard: bool,
    /// an array of the emoji guesses, in order
    pub guesses: Vec<String>,
}

/// Parses the Wordle score.
/// ## Example
/// ```
/// use parser::parse;
///
/// let resp = parse("Wordle 258 4/6*\n\n⬜⬜🟨⬜🟨\n⬜🟨⬜🟨⬜\n🟩⬜🟩⬜⬜\n🟩🟩🟩🟩🟩");
/// println!("{:#?}", resp);
/// ```
pub fn parse(input: &str) -> Result<ParsedWordle> {
    let inp = input.clone();
    let reg = Regex::new(
        r"Wordle (?P<day>\d{3}) (?P<score>\d)/6(?P<hard>\*?)\n\n(?P<guesses>([🟩🟨⬜⬛\n]*){1,6})",
    )
    .unwrap();

    let captures = reg.captures(inp);
    let captures = match captures {
        Some(c) => c,
        None => return Err(anyhow::anyhow!("Could not parse input")),
    };

    let guesses = captures
        .name("guesses")
        .unwrap()
        .as_str()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    return Ok(ParsedWordle {
        day: captures
            .name("day")
            .map_or("0", |m| m.as_str()),
        score: captures
            .name("score")
            .map_or("0", |m| m.as_str()),
        hard: captures.name("hard").map_or("", |m| m.as_str()).contains("*"),
        guesses,
    });
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use std::println as info;

    #[test]
    fn test_parse() {
        let parsed = parse("Wordle 258 4/6*\n\n⬜⬜🟨⬜🟨\n⬜🟨⬜🟨⬜\n🟩⬜🟩⬜⬜\n🟩🟩🟩🟩🟩")
                .unwrap();
        info!("{:#?}", parsed);

        assert_eq!(parsed.day, "258");
        assert_eq!(parsed.score, "4");
        assert_eq!(parsed.hard, true);
        assert_eq!(parsed.guesses.len(), 4);
        assert_eq!(parsed.guesses[3], "🟩🟩🟩🟩🟩");
    }
}
