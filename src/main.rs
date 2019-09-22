use std::io;
use std::fs;

/// 파일에서 정보를 가져온다.
fn get_json_content_on_file() -> json::JsonResult<json::JsonValue> {
    let file_contents = fs::read_to_string("data/program.json").unwrap();

    json::parse(&file_contents)
}

fn main() -> Result<(), io::Error> {
    let contents = get_json_content_on_file().unwrap();

    println!("{:?}", contents);
    Ok(())
}
