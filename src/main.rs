use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use clap::{App, Arg};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum GcPhase {
    Scavenge,
    MarkSweep,
    MarkCompact,
    Unknown
}

#[derive(Debug, PartialEq)]
enum Timing {
    Ok(String),
    Empty,
}

type Body = String;

#[derive(Debug)]
#[allow(dead_code)]
struct GcStats {
    phase: GcPhase,
    timing: Timing,
    body: Body,
}

impl GcStats {
    fn new(phase: GcPhase, timing: Timing, body: Body) -> GcStats {
        GcStats {
            phase: phase,
            timing: timing,
            // TODO: create GcBodyStats struct
            body: body,
        }
    }
}

fn parse_log(line: &str) -> GcStats {
    let targeted_elem_count = 6;
    let elements: Vec<_> = line.splitn(targeted_elem_count, " ").collect();

    if elements.len() < targeted_elem_count {
        println!(
            "Split failure target length {}, but got {}!",
            targeted_elem_count, 
            elements.len()
        ); 
        return GcStats::new(GcPhase::Unknown, Timing::Empty, String::from(line));
    }

    let mut timing: String = String::from(elements[2]);
    timing.push_str(&String::from(" "));
    timing.push_str(&String::from(elements[3]));
    timing.truncate(timing.len() - 1);
    let phase = elements[4];
    let body = Body::from(elements[5]);

    match phase {
        "Scavenge" => GcStats::new(GcPhase::Scavenge, Timing::Ok(timing), body),
        "Mark-sweep" => GcStats::new(GcPhase::MarkSweep, Timing::Ok(timing), body),
        "Mark-compact" => GcStats::new(GcPhase::MarkCompact, Timing::Ok(timing), body),
        _ => GcStats::new(GcPhase::Unknown, Timing::Empty, String::from(line))
    }
}

fn get_file_reader(log_file_path: String) -> BufReader<File> {
    let log_file_path = Path::new(&log_file_path);
    let log_file = File::open(log_file_path).unwrap();
    
    BufReader::new(log_file)
}

fn get_input_file_path() -> String {
    let path_arg_name = "path";
    let args = App::new("gc-log-parser")
        .arg(
            Arg::with_name(path_arg_name)
                .short("p")
                .long(path_arg_name)
                .help("Path to the log file")
                .takes_value(true)
                .required(true),
        ).get_matches();

    let raw_string = args.value_of(path_arg_name).unwrap();
    String::from(raw_string)
}

fn main() {
    let file_path = get_input_file_path();
    let file_reader: BufReader<File> = get_file_reader(file_path);
 
    file_reader.lines()
        .map(|line| parse_log(&line.unwrap()))
        .for_each(|gc_stats| {
            println!("{:?}", gc_stats);
        });
}

#[cfg(test)]
mod tests {
    use super::{parse_log, GcPhase, Timing};

    #[test]
    fn parse_log_should_handle_scavenge_phase() {
        let input = "[19278:0x5408db0]  44 ms: Scavenge 2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure";
        
        let stats = parse_log(input);

        assert_eq!(stats.timing, Timing::Ok(String::from("44 ms")));
        assert_eq!(stats.phase, GcPhase::Scavenge);
        assert_eq!(stats.body, String::from("2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure"));
    }
    
    #[test]
    fn parse_log_should_handle_marksweep_phase() {
        let input = "[19278:0x5408db0]  44 ms: Mark-sweep 2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure";
        
        let stats = parse_log(input);

        assert_eq!(stats.timing, Timing::Ok(String::from("44 ms")));
        assert_eq!(stats.phase, GcPhase::MarkSweep);
        assert_eq!(stats.body, String::from("2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure"));
    }
    
    #[test]
    fn parse_log_should_handle_markcompact_phase() {
        let input = "[19278:0x5408db0]  44 ms: Mark-compact 2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure";

        let stats = parse_log(input);

        assert_eq!(stats.timing, Timing::Ok(String::from("44 ms")));
        assert_eq!(stats.phase, GcPhase::MarkCompact);
        assert_eq!(stats.body, String::from("2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure"));
    }
    
    #[test]
    fn parse_log_should_handle_unknown_phase() {
        let input = "[19278:0x5408db0]  44 ms: UNKNOWN 2.3 (3.0) -> 1.9 (4.0) MB, 1.2 / 0.0 ms  (average mu = 1.000, current mu = 1.000) allocation failure";
        
        let stats = parse_log(input);

        assert_eq!(stats.timing, Timing::Empty);
        assert_eq!(stats.phase, GcPhase::Unknown);
        assert_eq!(stats.body, String::from(input));
    }
    
    #[test]
    fn parse_log_should_handle_incorrect_line_structure() {
        let input = "[19278:0x5408db0]  44 ms";

        let stats = parse_log(input);

        assert_eq!(stats.timing, Timing::Empty);
        assert_eq!(stats.phase, GcPhase::Unknown);
        assert_eq!(stats.body, String::from(input));
    }
}
