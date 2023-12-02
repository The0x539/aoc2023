use std::io::Write;

use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};

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

fn pick_block(blocks: Vec<ElementRef>) -> Result<ElementRef> {
    if blocks.is_empty() {
        bail!("no code blocks");
    } else if blocks.len() == 1 {
        return Ok(blocks.into_iter().next().unwrap());
    }

    for block in blocks {
        println!("{}", block.inner_html());

        if prompt("is this the test input?")? {
            return Ok(block);
        }
    }
    bail!("no match");
}

fn sel(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

fn get_test_output(part: ElementRef) -> Option<String> {
    part.select(&sel("code > em"))
        .last()
        .map(|n| n.inner_html())
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

    let cookie = include_str!("../session").trim();
    let client = Client::new();

    let http_get = |url: &str| -> Result<String> {
        client
            .get(url)
            .header("Cookie", cookie)
            .header("User-Agent", "The0x539's AoC scraper")
            .send()
            .context("transport error")?
            .error_for_status()
            .context("http error")?
            .text()
            .context("body error")
    };

    let base_url = format!("https://adventofcode.com/{year}/day/{day}");
    let html = http_get(&base_url)?;

    let document = Html::parse_document(&html);

    let desc_selector = sel(".day-desc");
    let mut parts = document.select(&desc_selector);

    let part1 = parts.next().context("no part 1 description")?;

    let blocks = part1.select(&sel("pre > code")).collect::<Vec<_>>();
    let test_input = pick_block(blocks)?.inner_html();

    let test_output_1 = get_test_output(part1).context("could not find part 1 test output")?;

    let test_output_2 = if let Some(part2) = parts.next() {
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
