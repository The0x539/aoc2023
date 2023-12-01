use std::io::Write;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use soup::prelude::*;

fn crate_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()?
        .parent()?
        .parent()?
        .parent()?
        .to_owned()
        .into()
}

fn prompt(msg: &str) -> Result<bool> {
    let mut input = String::new();
    loop {
        print!("{msg} [y/n] ");
        std::io::stdout().flush()?;

        input.clear();
        std::io::stdin().read_line(&mut input)?;
        match input.as_bytes().get(0) {
            Some(b'y' | b'Y') => return Ok(true),
            Some(b'n' | b'N') => return Ok(false),
            _ => continue,
        }
    }
}

fn pick_block<N: NodeExt>(blocks: Vec<N>) -> Result<N> {
    for block in blocks {
        println!("{}", block.text());

        if prompt("is this the test input?")? {
            return Ok(block);
        }
    }
    bail!("no match");
}

fn get_test_output<N: QueryBuilderExt>(part: &N) -> Option<String> {
    part.tag("code")
        .find_all()
        .flat_map(|n| n.tag("em"))
        .last()
        .map(|n| n.text())
}

fn what_year_is_it() -> i32 {
    use chrono::prelude::*;
    let tz = FixedOffset::west_opt(5 * 3600).unwrap();
    let date = Utc::now().with_timezone(&tz);
    if date.month() == 12 {
        date.year()
    } else {
        // grab from last year
        date.year() - 1
    }
}

fn main() -> Result<()> {
    let cwd = std::env::current_dir().context("no cwd")?;

    let day = cwd
        .iter()
        .last()
        .context("no last segment of cwd")?
        .to_str()
        .context("invalid UTF-8")?
        .strip_prefix("day")
        .context("cwd does not start with `day`")?
        .parse::<u8>()?;

    let year = what_year_is_it();

    let cookie = std::fs::read_to_string(crate_dir().context("crate dir fail")?.join("session"))
        .context("couldn't read session")?;
    let client = Client::new();

    let http_get = |url: &str| -> Result<String> {
        client
            .get(url)
            .header("Cookie", &cookie)
            .header("User-Agent", "The0x539's AoC scraper")
            .send()?
            .error_for_status()?
            .text()
            .map_err(From::from)
    };

    let base_url = format!("https://adventofcode.com/{year}/day/{day}");
    let html = http_get(&base_url)?;

    // TODO: use kuchikiki
    let soup = Soup::new(&html);
    let parts = soup.class("day-desc").find_all().collect::<Vec<_>>();

    let part1 = parts.get(0).context("no part 1 description")?;

    let blocks = part1.tag("pre").find_all().collect::<Vec<_>>();

    let test_input = match blocks.len() {
        0 => bail!("no <pre> elements"),
        1 => blocks[0].text(),
        _ => pick_block(blocks)?.text(),
    };

    let test_output_1 = get_test_output(part1).context("could not find part 1 test output")?;

    let test_output_2 = if let Some(part2) = parts.get(1) {
        get_test_output(part2).context("could not find part 2 test output")?
    } else {
        "0".to_owned()
    };

    let test_output = format!("{test_output_1}\n{test_output_2}");

    println!("expected test output: {test_output_1} {test_output_2}");

    let real_input = http_get(&(base_url + "/input")).context("input get fail")?;

    std::fs::write(cwd.join("test.txt"), test_input)?;
    std::fs::write(cwd.join("test.out.txt"), test_output)?;
    std::fs::write(cwd.join("input.txt"), real_input)?;

    Ok(())
}
