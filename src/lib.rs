use regex::Regex;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct ExtractResult {
    pub title: String,
    pub desc: String,
}

impl Serialize for ExtractResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut res = serializer.serialize_struct("ExtractResult", 2)?;
        res.serialize_field("title", &self.title)?;
        res.serialize_field("desc", &self.desc)?;
        res.end()
    }
}

#[tokio::main]
pub async fn extract_td(link: &str) -> Result<ExtractResult, Box<dyn std::error::Error>> {
    let body = reqwest::get(link).await?.text().await?.replace("\"", "'");
    let result = extract_title_and_desc(body);
    Ok(result)
}

fn extract_title_and_desc(input: String) -> ExtractResult {
    let input_str = input.as_str();
    let desc_re = Regex::new(r"<meta.?name='description'.?content='([\s\S]*?)'.*?>").unwrap();
    let title_re = Regex::new(r"<title>([\s\S]*?)</title>").unwrap();
    ExtractResult {
        title: String::from(extract(input_str, title_re)),
        desc: String::from(extract(input_str, desc_re)),
    }
}

fn extract(input: &str, re: Regex) -> &str {
    let caps = re.captures(input);
    match caps {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()).trim(),
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let result = extract_title_and_desc(String::from(""));
        assert_eq!(result.title, String::from(""));
        assert_eq!(result.desc, String::from(""));
    }

    #[test]
    fn test_extd() {
        let result = extract_td("https://www.rust-lang.org/");
        match result {
            Ok(r) => println!("{:?}", r),
            _ => (),
        }
    }
}
