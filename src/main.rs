use std::io;
use std::fs;
use json::{JsonValue, JsonResult};

struct Session {
    day: String,
    title: String,
    tags: String,
    description: String,
    speakers: String,
    pdf_url: String,
    video_url: String,
}

/// 파일에서 정보를 가져온다.
fn get_json_content_on_file() -> JsonResult<JsonValue> {
    let file_contents = fs::read_to_string("data/program.json").unwrap();

    json::parse(&file_contents)
}

/// Session 을 정리한다.
fn get_sessions(contents: &JsonValue) -> Vec<Session> {
    let mut _result: Vec<Session> = Vec::new();

    for time in contents.members() {
        let sessions_list = &time["sessions"];

        for session in sessions_list.members() {
            let _session = Session {
                day: session["days"]["display"].to_string(),
                title: session["title"].to_string(),
                tags: get_tag_list(&session["tags"]),
                description: session["description"].to_string(),
                speakers: get_speaker_list(&session["speakers"]),
                pdf_url: session["pdfUrl"].to_string(),
                video_url: session["videoUrl"].to_string(),
            };

            _result.push(_session);
        }
    }

    _result
}

fn get_tag_list(tags: &JsonValue) -> String {
    let mut _tags: Vec<String> = Vec::new();

    for tag in tags.members() {
        if _tags.len() > 1 {
            _tags.push(format!(", "));
        }

        _tags.push(tag.to_string().replace(" ", ""))
    }

    format!("{}", _tags.join(""))
}

fn get_speaker_list(speakers: &JsonValue) -> String {
    let mut _speakers: Vec<String> = Vec::new();

    for speaker in speakers.members() {
        if _speakers.len() > 1 {
            _speakers.push(format!(", "));
        }

        _speakers.push(format!("{} <{}>", speaker["name"], speaker["org"]));
    }

    format!("{}", _speakers.join(""))
}

fn main() -> Result<(), io::Error> {
    let contents = get_json_content_on_file().unwrap();

    let day1 = get_sessions(&contents["day1"]);
    let day2 = get_sessions(&contents["day2"]);

    let mut sessions = vec![];
    sessions.extend(&day1);
    sessions.extend(&day2);

    println!("{:?}", sessions.len());

    Ok(())
}
