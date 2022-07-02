#[derive(Debug, PartialEq)]
enum GcPhase {
    Scavange,
    MarkSweep,
    MarkCompact,
    Unknown
}

type Body = String;

fn parse_log(line: &str) -> (GcPhase, Body) {
    let elements: Vec<_> = line.splitn(2, " ").collect();

    if elements.len() == 1 {
        return (GcPhase::Unknown, Body::from(line))
    }

    let phase = elements[0];
    let body = Body::from(elements[1]);

    match phase {
        "Scavange" => (GcPhase::Scavange, body.to_string()),
        "MarkSweep" => (GcPhase::MarkSweep, body.to_string()),
        "MarkCompact" => (GcPhase::MarkCompact, body.to_string()),
        _ => (GcPhase::Unknown, body.to_string())
    }
}

fn main() {
    println!("GcPhase parser");
}

#[cfg(test)]
mod tests {
    use super::{parse_log, GcPhase};

    #[test]
    fn parse_log_should_handle_scavange_phase() {
        let (phase, body) = parse_log("Scavange a body");
        assert_eq!(phase, GcPhase::Scavange);
        assert_eq!(body, String::from("a body"));
    }
    
    #[test]
    fn parse_log_should_handle_marksweep_phase() {
        let (phase, body) = parse_log("MarkSweep a body");
        assert_eq!(phase, GcPhase::MarkSweep);
        assert_eq!(body, String::from("a body"));
    }
    
    #[test]
    fn parse_log_should_handle_markcompact_phase() {
        let (phase, body) = parse_log("MarkCompact a body");
        assert_eq!(phase, GcPhase::MarkCompact);
        assert_eq!(body, String::from("a body"));
    }
    
    #[test]
    fn parse_log_should_handle_unknown_phase() {
        let (phase, body) = parse_log("CCDKJSJD a body");
        assert_eq!(phase, GcPhase::Unknown);
        assert_eq!(body, String::from("a body"));
    }
    
    #[test]
    fn parse_log_should_return_unknown_when_input_has_one_elem() {
        let one_elem_input = "CCDKJSJD";
        let (phase, body) = parse_log(one_elem_input);
        assert_eq!(phase, GcPhase::Unknown);
        assert_eq!(body, String::from(one_elem_input));
    }
}
