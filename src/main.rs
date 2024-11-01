#![doc = include_str!("../README.md")]
use std::io::{Read, Write};
use std::process::ExitCode;

use clap::Parser;

static FONT_INFO: &str = r#"
-> Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)
(http://studenten.freepage.de/meph/ascii/ascii/editor/_index.htm)
-> Defined: ASCII code alphabet
-> Uppercase characters only.
ScarecrowsASCIIArtArchive1.0.txt
From: "Sub-Zero" <bodom@papaya.ucs.indiana.edu>
"Here's a font I've been working on lately. Can someone make the V, Q, and X
look better? Also, the B, P, and R could use an improvement too.
Oh, here it is." 
 ______ __  __ ______ ______ ______  
/\__  _\\ \_\ \\  == \\  ___\\___  \ 
\/_/\ \/ \____ \\  _-/ \  __\/_/  /__
   \ \_\\/\_____\\_\  \ \_____\\_____\
    \/_/ \/_____//_/   \/_____//_____/
---------------------------------"#;

/// Tool for generating ascii art with Sub-Zero font
#[derive(Debug, Clone, Parser)]
#[clap(version, before_help(FONT_INFO), before_long_help(FONT_INFO))]
struct Cli {
    /// Text to convert
    text: Vec<String>,

    /// Treat input as an input file instead of text to convert (use - for stdin)
    #[clap(short, long, conflicts_with("text"))]
    input: Option<String>,

    /// Number of spaces for the space character
    #[clap(short, long, default_value = "5")]
    spaces: usize,

    /// Number of spaces between non-space characters
    #[clap(short, long, default_value = "2")]
    between: usize,

    /// Squash the characters together (put more `-S` for more squashing)
    #[clap(short = 'S', long, action(clap::ArgAction::Count))]
    squash: u8,
}

fn main() -> ExitCode {
    let args = Cli::parse();
    if args.squash > 3 {
        eprintln!("fatal: you are squashing too much");
        return ExitCode::FAILURE;
    }
    if args.text.is_empty() && args.input.is_none() {
        eprintln!("fatal: no input provided. See --help for usage");
        return ExitCode::FAILURE;
    }

    let input_text = match args.input {
        Some(file) => {
            if file == "-" {
                let mut buffer = String::new();
                if let Err(e) = std::io::stdin().read_to_string(&mut buffer) {
                    eprintln!("fatal: could not read from stdin: {}", e);
                    return ExitCode::FAILURE;
                }
                buffer
            } else {
                match std::fs::read_to_string(&file) {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("fatal: could not read from file '{}': {}", file, e);
                        return ExitCode::FAILURE;
                    }
                }
            }
        }
        None => args.text.join(" "),
    };

    print_art(&input_text, args.spaces, args.between, args.squash as usize);

    ExitCode::SUCCESS
}

#[rustfmt::skip]
mod data {
    pub static DATA: &[&[u8]] = &[
         br" ______  ",br" ______  ",br" ______  ",br" _____   ",br" ______  ",br" ______  ",br" ______  ",br" __  __  ",br" __  ",br"    __  ",br#" __  __  "#,br" __      ",br#" __    __  "#,br#" __   __  "#,br" ______  ",br" ______  ",br" ______  ",br" ______  ",br" ______  ",br" ______  ",br" __  __  ",br" __   __ ",br#" __     __  "#,br" __  __  ",br" __  __  ",br" ______  ",
         br"/\  __ \ ",br"/\  == \ ",br"/\  ___\ ",br"/\  __-. ",br"/\  ___\ ",br"/\  ___\ ",br"/\  ___\ ",br"/\ \_\ \ ",br"/\ \ ",br"   /\ \ ",br#"/\ \/ /  "#,br"/\ \     ",br#"/\ "-./  \ "#,br#"/\ "-.\ \ "#,br"/\  __ \ ",br"/\  == \ ",br"/\  __ \ ",br"/\  == \ ",br"/\  ___\ ",br"/\__  _\ ",br"/\ \/\ \ ",br"/\ \ / / ",br#"/\ \  _ \ \ "#,br"/\_\_\_\ ",br"/\ \_\ \ ",br"/\___  \ ",
         br"\ \  __ \",br"\ \  __< ",br"\ \ \____",br"\ \ \/\ \",br"\ \  __\ ",br"\ \  __\ ",br"\ \ \__ \",br"\ \  __ \",br"\ \ \",br"  _\_\ \",br#"\ \  _"-."#,br"\ \ \____",br#"\ \ \-./\ \"#,br#"\ \ \-.  \"#,br"\ \ \/\ \",br"\ \  _-/ ",br"\ \ \/\_\",br"\ \  __< ",br"\ \___  \",br"\/_/\ \/ ",br"\ \ \_\ \",br"\ \ \'/  ",br#"\ \ \/ ".\ \"#,br"\/_/\_\/_",br"\ \____ \",br"\/_/  /__",
          br"\ \_\ \_\",br"\ \_____\",br"\ \_____\",br"\ \____- ",br"\ \_____\",br"\ \_\    ",br"\ \_____\",br"\ \_\ \_\",br"\ \_\",br"/\_____\",br#"\ \_\ \_\"#,br"\ \_____\",br#"\ \_\ \ \_\"#,br#"\ \_\\"\_\"#,br"\ \_____\",br"\ \_\    ",br"\ \___\_\",br"\ \_\ \_\",br"\/\_____\",br"  \ \_\  ",br"\ \_____\",br"\ \__|   ",br#"\ \__/".~\_\"#,br" /\_\/\_\",br"\/\_____\",br" /\_____\",
          br" \/_/\/_/",br" \/_____/",br" \/_____/",br" \/____/ ",br" \/_____/",br" \/_/    ",br" \/_____/",br" \/_/\/_/",br" \/_/",br"\/_____/",br#" \/_/\/_/"#,br" \/_____/",br#" \/_/  \/_/"#,br#" \/_/ \/_/"#,br" \/_____/",br" \/_/    ",br" \/___/_/",br" \/_/ /_/",br" \/_____/",br"   \/_/  ",br" \/_____/",br" \/_/    ",br#" \/_/   \/_/"#,br" \/_/\/_/",br" \/_____/",br" \/_____/",
    ];
}
use data::*;

const CHARS: usize = 26;
const SPACE: u8 = b' ';

fn print_art(input: &str, spaces: usize, between: usize, squash: usize) {
    for input_line in input.lines() {
        let mut output_lines = [Vec::new(), Vec::new(), Vec::new(), vec![SPACE], vec![SPACE]];

        let mut first = true;

        for c in input_line.chars() {
            if c < '\n' || c == 0x7f as char || c == 0xff as char {
                continue;
            }
            let char_to_put = if c.is_ascii_lowercase() {
                c as usize - 'a' as usize
            } else if c.is_ascii_uppercase() {
                c as usize - 'A' as usize
            } else {
                // put spaces
                for line in &mut output_lines {
                    line.extend((0..spaces).map(|_| SPACE));
                }
                first = true;
                continue;
            };
            let next_glyphs = [
                DATA[char_to_put],
                DATA[CHARS + char_to_put],
                DATA[2 * CHARS + char_to_put],
                DATA[3 * CHARS + char_to_put],
                DATA[4 * CHARS + char_to_put],
            ];
            if first || squash == 0 {
                for (line, next) in std::iter::zip(output_lines.iter_mut(), next_glyphs.into_iter())
                {
                    if !first {
                        line.extend((0..between).map(|_| SPACE));
                    }
                    line.extend(next);
                }
            } else {
                for (line, next) in std::iter::zip(output_lines.iter_mut(), next_glyphs.into_iter())
                {
                    let mut popped = 0;
                    let line_len = line.len();
                    while popped < squash {
                        if can_overwrite(line[line_len - popped - 1], next[0]) {
                            popped += 1;
                        } else {
                            break;
                        }
                    }
                    let start = squash - popped;
                    for i in 0..popped {
                        let line_i = line_len - popped + i;
                        let next_char = next[start + i];
                        if can_overwrite(line[line_i], next_char) {
                            line[line_i] = next_char;
                        }
                    }
                    line.extend(&next[squash..]);
                }
            }

            first = false;
        }

        for line in output_lines {
            std::io::stdout().write_all(&line).unwrap();
            println!();
        }
    }
}

/// Can b overwrite a?
fn can_overwrite(a: u8, b: u8) -> bool {
    // space can be overwritten by anything
    if a == SPACE {
        return true;
    }
    if b == SPACE {
        return false;
    }
    // \ or / or other can generally overwrite _
    if a == b'_' {
        return true;
    }
    false
}
