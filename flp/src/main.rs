mod style_ext;

use dotenv;
use termion::input::TermRead;
use std::{
    env,
    io::{self, Read, Write, BufReader, BufRead},
    process::{Command, Stdio},
};
use style_ext::StyleExt;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let envs: Vec<(String, String)> = ["SITE", "PORT", "PASSWORD"]
        .into_iter()
        .map(|k| (k.to_string(), dotenv::var(k).unwrap()))
        .collect();

    let mut bc = Command::new("bc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("unable to start bc");
    let mut stdin = bc.stdin.take().unwrap();
    let mut stdout = bc.stdout.take().unwrap();

    println!("{stdout:?}");

    let cmds = [
        "3+4\n",
        "12*12\n",
        "quit\n",
    ];
    
    for c in cmds {
        println!("{:?}", stdin.write_all(c.as_bytes()));
    }

    while let Some(l) = stdout.read_line().unwrap() {
        if l.len() == 0 {
            break;
        }
        println!("{l}");
    }
    

    Ok(())
}

/*
[file_name.vid]
Which catagory does this content fall under?
Anime Movies TV
If its ambiguous (eg. ova's) and you aren't sure, ask me first
*/
