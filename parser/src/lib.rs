//! A Wordle result parser

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace1, newline, space1, u16, u8},
    combinator::{map, opt},
    error::Error,
    multi::{many_m_n, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Err, IResult,
};

const WORD_LENGTH: usize = 5;

/// A guess character
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum GuessChar {
    Green,
    Yellow,
    White,
    Black,
}

/// A line of guess characters
pub type GuessLine = [GuessChar; WORD_LENGTH];

/// A list of guesses
pub type Guesses = Vec<GuessLine>;

// A wordle game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wordle {
    /// the score, out of six for the game
    pub score: u8,
    /// the day of the game
    pub day: u16,
    /// whether the game was played in hard mode
    pub hard: bool,
    /// an array of the emoji guesses, in order
    pub guesses: Guesses,
}

impl<'a> TryFrom<&'a str> for Wordle {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parse(value)?.1)
    }
}

/// Parse the day number
///
/// Ex: 123[space] -> 123
fn parse_day(input: &str) -> IResult<&str, u16> {
    terminated(u16, space1)(input)
}

/// Parse the score and whether it was hard
///
/// Ex: 4/6*[space+] -> 4, true
fn parse_score_hard(input: &str) -> IResult<&str, (u8, bool)> {
    terminated(
        separated_pair(u8, tag("/6"), map(opt(char('*')), |c| c.is_some())),
        multispace1,
    )(input)
}

/// Parse a guess emoji character
///
/// Ex: 🟩 -> GuessChar::Green
fn parse_guess_char(input: &str) -> IResult<&str, GuessChar> {
    alt((
        map(char('🟩'), |_| GuessChar::Green),
        map(char('🟨'), |_| GuessChar::Yellow),
        map(char('⬜'), |_| GuessChar::White),
        map(char('⬛'), |_| GuessChar::Black),
    ))(input)
}

/// Parse an entire line of guess characters
///
/// Ex: 🟩🟩🟩🟩🟩 -> [🟩🟩🟩🟩🟩]
fn parse_guess_line(input: &str) -> IResult<&str, GuessLine> {
    map(many_m_n(5, 5, parse_guess_char), |chars| unsafe {
        // SAFETY: guaranteed to have only 5 characters from parser, meaning this always succeeds
        chars.try_into().unwrap_unchecked()
    })(input)
}

/// Parse an entire matrix of guesses
///
/// Ex:
/// 🟩🟩🟩🟩🟩\n🟩🟩🟩🟩🟩 ->
/// ```
/// [
///     [ 🟩,🟩,🟩,🟩,🟩 ],
///     [ 🟩,🟩,🟩,🟩,🟩 ]
/// ]
/// ```
fn parse_guesses(input: &str) -> IResult<&str, Guesses> {
    separated_list1(newline, parse_guess_line)(input)
}

/// Parse full wordle content
fn parse_data(input: &str) -> IResult<&str, Wordle> {
    let (rem, (day, (score, hard), guesses)) =
        tuple((parse_day, parse_score_hard, parse_guesses))(input)?;

    Ok((
        rem,
        Wordle {
            day,
            score,
            hard,
            guesses,
        },
    ))
}

/// Parses the Wordle score.
/// ## Example
/// ```
/// use parser::parse;
///
/// let resp = parse("Wordle 258 4/6*\n\n⬜⬜🟨⬜🟨\n⬜🟨⬜🟨⬜\n🟩⬜🟩⬜⬜\n🟩🟩🟩🟩🟩");
/// println!("{:#?}", resp);
/// ```
pub fn parse(input: &str) -> IResult<&str, Wordle> {
    preceded(tag("Wordle "), parse_data)(input)
}

#[cfg(test)]
mod tests {
    use crate::{GuessChar, Wordle};

    #[test]
    fn test_parse() {
        let parsed =
            Wordle::try_from("Wordle 258 4/6*\n\n⬜⬜🟨⬜🟨\n⬜🟨⬜🟨⬜\n🟩⬜🟩⬜⬜\n🟩🟩🟩🟩🟩")
                .unwrap();
        println!("{:#?}", parsed);

        assert_eq!(
            parsed,
            Wordle {
                day: 258,
                score: 4,
                hard: true,
                guesses: vec![
                    [
                        GuessChar::White,
                        GuessChar::White,
                        GuessChar::Yellow,
                        GuessChar::White,
                        GuessChar::Yellow
                    ],
                    [
                        GuessChar::White,
                        GuessChar::Yellow,
                        GuessChar::White,
                        GuessChar::Yellow,
                        GuessChar::White
                    ],
                    [
                        GuessChar::Green,
                        GuessChar::White,
                        GuessChar::Green,
                        GuessChar::White,
                        GuessChar::White
                    ],
                    [
                        GuessChar::Green,
                        GuessChar::Green,
                        GuessChar::Green,
                        GuessChar::Green,
                        GuessChar::Green
                    ]
                ],
            }
        );
    }
}
