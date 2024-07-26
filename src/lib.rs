use regex::Regex;
use wasm_bindgen::prelude::*;

fn word_matches(word: &str, text: &str) -> Result<Vec<String>, regex::Error> {
    let re = Regex::new(&format!(r"\b[^.!?]*?{}[^.!?]*?[.!?]", word))?;
    let mut matches = vec![];
    for mat in re.find_iter(text) {
        matches.push(mat.as_str().to_string());
    }
    Ok(matches)
}

#[wasm_bindgen]
pub fn word_search(word: &str, text: &str) -> Result<Vec<String>, JsValue> {
    word_matches(word, text).map_err(|e| JsValue::from_str(&format!("Regex error : {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_search_test() {
        let text = r"This is hoge. This is bar. This is foo. But hoge is best. Big Hoge. HOGE. Hoge means hoge.hogehoge.";
        let result = word_search("hoge", text).expect("Failed word_search");
        println!("Result is {:?}", &result);
        assert_eq!(result, vec!["This is hoge.", "But hoge is best.", "Hoge means hoge.", "hogehoge."]);
    }
}
