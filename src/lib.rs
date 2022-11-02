use regex::Regex;

pub struct ExtractResult {
    title: String,
    desc: String,
}

#[tokio::main]
pub async fn extract_td(link: &str) -> Result<ExtractResult, Box<dyn std::error::Error>> {
    let body = reqwest::get(link).await?.text().await?.replace("\"", "'");
    let result = extract_title_and_desc(body);
    Ok(result)
}

fn extract_title_and_desc(input: String) -> ExtractResult {
    let input_str = input.as_str();
    let desc_re = Regex::new(r"<meta.?name='description'.?content='(.*?)'.*?>").unwrap();
    let title_re = Regex::new("<title>(.*?)</title>").unwrap();
    ExtractResult {
        title: String::from(extract(input_str, title_re)),
        desc: String::from(extract(input_str, desc_re)),
    }
}

fn extract(input: &str, re: Regex) -> &str {
    let caps = re.captures(input);
    match caps {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
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
}
