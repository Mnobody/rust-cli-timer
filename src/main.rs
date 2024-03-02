use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use regex::Regex;
use std::io::{stdout, Write};
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let duration = std::env::args().nth(1).expect("no duration given");

    countdown(duration.clone());

    notify(duration.clone());

    println!();
    process::exit(0);
}

fn notify(duration: String) {
    Notification::new()
        .summary("Done")
        .body(&duration)
        .icon("terminal")
        .show()
        .unwrap();
}

fn countdown(duration: String) {
    let seconds = duration_in_seconds(duration.clone());
    let pb = ProgressBar::new(seconds.try_into().unwrap());

    for second in (0..seconds).rev() {
        pb.set_style(
            ProgressStyle::with_template(
                &format!("{elapsed_precise}   {{wide_bar}}  ", elapsed_precise = readable_countdown(second)),
            ).unwrap(),
        );

        pb.set_position(second.try_into().unwrap());

        stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }

    pb.finish();
    pb.set_position(0);
}

fn readable_countdown(seconds: i32) -> String {
    let h = (seconds / 60) / 60;
    let m = (seconds / 60) % 60;
    let s = seconds % 60;

    format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
}

fn duration_in_seconds(duration: String) -> i32 {
    hours(duration.clone()) * 3600 + minutes(duration.clone()) * 60 + seconds(duration.clone())
}

fn hours(duration: String) -> i32 {
    quantity(r"(\d+)h".to_string(), duration)
}

fn minutes(duration: String) -> i32 {
    quantity(r"(\d+)m".to_string(), duration)
}

fn seconds(duration: String) -> i32 {
    quantity(r"(\d+)s".to_string(), duration)
}

fn quantity(regex: String, duration: String) -> i32 {
    let regex = Regex::new(&regex).unwrap();

    if regex.is_match(&duration) {
        let caps = regex.captures(&duration).unwrap();

        return caps.get(1).unwrap().as_str().parse().unwrap();
    }

    0
}
