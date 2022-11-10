use anyhow::{anyhow, Context, Result};
use chrono::{Datelike, FixedOffset, NaiveDate, TimeZone, Utc};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE};
use reqwest::redirect::Policy;
use std::convert::{TryFrom, TryInto};
use std::env::home_dir;
use std::fs::{self, read_to_string, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::aocdata::Part;
use super::puzzle::{PuzzleDay, PuzzleYear};

const DECEMBER: u32 = 12;
const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 3600;

pub fn latest_event_year() -> PuzzleYear {
    let now = FixedOffset::east(RELEASE_TIMEZONE_OFFSET).from_utc_datetime(&Utc::now().naive_utc());

    if now.month() < DECEMBER {
        (now.year() - 1).try_into().expect("to be valid")
    } else {
        (now.year()).try_into().expect("to be valid")
    }
}

pub fn current_event_day(year: PuzzleYear) -> Option<PuzzleDay> {
    let now = FixedOffset::east(RELEASE_TIMEZONE_OFFSET).from_utc_datetime(&Utc::now().naive_utc());

    if now.month() == DECEMBER && now.year() == year.get() as i32 {
        now.day().try_into().ok()
    } else {
        None
    }
}

pub fn puzzle_unlocked(year: PuzzleYear, day: PuzzleDay) -> bool {
    let timezone = FixedOffset::east(RELEASE_TIMEZONE_OFFSET);
    let now = timezone.from_utc_datetime(&Utc::now().naive_utc());
    let unlock_time = timezone
        .from_local_datetime(
            &NaiveDate::from_ymd(year.get() as i32, DECEMBER, day.get() as u32).and_hms(0, 0, 0),
        )
        .single();

    if let Some(time) = unlock_time {
        now.signed_duration_since(time).num_milliseconds() >= 0
    } else {
        false
    }
}

fn puzzle_day_year(
    opt_year: Option<PuzzleYear>,
    opt_day: Option<PuzzleDay>,
) -> Result<(PuzzleYear, PuzzleDay)> {
    let year = opt_year.unwrap_or_else(latest_event_year);
    let day = opt_day
        .or_else(|| current_event_day(year))
        .ok_or_else(|| anyhow!("Could not infer puzzle day for {}.", year))?;

    if !puzzle_unlocked(year, day) {
        return Err(anyhow!("Puzzle {} of {} is still locked.", day, year));
    }

    Ok((year, day))
}

fn build_client(session_cookie: &str, content_type: &str) -> Result<Client> {
    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim()))?;
    let content_type_header = HeaderValue::from_str(content_type).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_type_header);

    let client = Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()?;
    Ok(client)
}

pub fn download_input(session_cookie: &str, year: PuzzleYear, day: PuzzleDay) -> Result<String> {
    // let (year, day) = puzzle_day_year(opt_year, opt_day)?;

    eprintln!("Downloading input for day {}, {}...", day, year);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let content_type = "text/plain";
    let puzzle_input = build_client(session_cookie, content_type)?
        .get(&url)
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())?;

    // eprintln!("Saving puzzle input to \"{}\"...", filename);
    // OpenOptions::new()
    //     .write(true)
    //     .create_new(true)
    //     .open(filename)?
    //     .write(puzzle_input.as_bytes())?;

    eprintln!("Done!");
    Ok(puzzle_input)
}

pub fn read_input(year: PuzzleYear, day: PuzzleDay) -> Option<String> {
    let dir = get_puzzle_dir(year, day);
    let path = dir.join("input.txt");
    read_to_string(path).ok()
}

pub fn get_input(session_cookie: &str, year: PuzzleYear, day: PuzzleDay) -> Result<String> {
    if let Some(cached) = read_input(year, day) {
        Ok(cached)
    } else {
        let input = download_input(session_cookie, year, day)?;
        let path = get_puzzle_dir(year, day).join("input.txt");
        let _ = fs::write(&path, &input).with_context(|| {
            eprintln!("Unable to cache results to file {}", path.to_string_lossy());
            "Caching input to filesystem"
        })?;
        Ok(input)
    }
}

pub fn get_puzzle_dir(year: PuzzleYear, day: PuzzleDay) -> PathBuf {
    let path: PathBuf = format!("./aoc-data/{year}/{day}").into();
    fs::create_dir_all(&path).expect("Directory to be creatable");
    path
}

pub fn read_session_cookie() -> Result<String> {
    let mut home = dirs::home_dir().ok_or(anyhow!("No home directory found"))?;
    let path = home.join(".adventofcode.session");
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Reading session file, {}", path.to_string_lossy()))?;
    Ok(contents)
}

pub fn submit_answer(
    session_cookie: &str,
    year: PuzzleYear,
    day: PuzzleDay,
    part: Part,
    answer: &str,
) -> Result<String> {
    // let (year, day) = puzzle_day_year(opt_year, opt_day)?;
    eprintln!(
        "Submitting answer: {answer} for part {:?}, day {}, {}...",
        part, day, year
    );
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let content_type = "application/x-www-form-urlencoded";
    let response = build_client(session_cookie, content_type)?
        .post(&url)
        .body(format!("level={}&answer={}", part.get_level(), answer))
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())?;

    let result = Regex::new(r"(?i)(?s)<main>(?P<main>.*)</main>")
        .unwrap()
        .captures(&response)
        .ok_or(anyhow!("Failed to parse response"))?
        .name("main")
        .unwrap()
        .as_str()
        .to_owned();

    let too_recent = Regex::new(r"you gave an answer too recently")
        .unwrap()
        .is_match(&result);
    if too_recent {
        return Err(anyhow!("{}", &result));
    }

    Ok(result)
}

pub fn read_puzzle(session_cookie: &str, year: PuzzleYear, day: PuzzleDay) -> Result<String> {
    // let (year, day) = puzzle_day_year(opt_year, opt_day)?;

    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let content_type = "text/html";
    let response = build_client(session_cookie, content_type)?
        .get(&url)
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())?;

    let description = Regex::new(r"(?i)(?s)<main>(?P<main>.*)</main>")
        .unwrap()
        .captures(&response)
        .ok_or(anyhow!("Failed to parse puzzle description page"))?
        .name("main")
        .unwrap()
        .as_str()
        .to_owned();

    Ok(description)
}
