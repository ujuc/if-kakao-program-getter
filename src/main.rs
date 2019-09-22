use std::io;
use std::fs;
use serde_json::Value;

/// 파일에서 정보를 가져온다.
fn get_json_content_on_file() -> Result<Value, io::Error> {
    let file_contents = fs::read_to_string("data/program.json")?;
    let contents: Value = serde_json::from_str(&file_contents).unwrap();

    Ok(contents)
}

fn main() -> Result<(), io::Error> {
    let contents = get_json_content_on_file()?;

    println!("{:?}", contents);
    Ok(())
}
