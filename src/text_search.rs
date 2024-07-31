use regex::Regex;
use wasm_bindgen::prelude::*;

/// The `word_matches` function searches for sentences in a text that contain a specified word.
///
/// # Arguments
///
/// * `word`: The target word to search for.
/// * `text`: The text in which to perform the search.
///
/// # Returns
///
/// * `Ok(Vec<String>)`: A list of sentences containing the word. If the word does not exist in the text, it returns an empty list.
/// * `Err(regex::Error)`: Returns an error if the creation of the regular expression fails.
///
/// # Example
///
/// ```
/// let text = "Hello, world! I love the world. Thank you. World...";
/// let matches = word_matches("world", text);
/// assert_eq!(matches, Ok(vec!["Hello, world!", "I love the world."]));
/// ```
fn word_matches(word: &str, text: &str) -> Result<Vec<String>, regex::Error> {
    let re = Regex::new(&format!(r"\b[^.!?]*?{}[^.!?]*?[.!?]", word))?;
    let mut matches = vec![];
    for mat in re.find_iter(text) {
        matches.push(mat.as_str().to_string());
    }
    Ok(matches)
}

/// The `word_search` function is a wrapper for the `word_matches` function that handles errors in a way suitable for WebAssembly.
///
/// # Arguments
///
/// * `word`: The target word to search for.
/// * `text`: The text in which to perform the search.
///
/// # Returns
///
/// * `Ok(Vec<String>)`: A list of sentences containing the word. If the word does not exist in the text, it returns an empty list.
/// * `Err(JsValue)`: Returns a JavaScript error if the creation of the regular expression fails.
///
/// # Example
///
/// ```
/// let text = "Hello, world! I love the world. Thank you. World...";
/// let matches = word_search("world", text);
/// assert_eq!(matches, Ok(vec!["Hello, world!", "I love the world."]));
/// ```
#[wasm_bindgen]
pub fn word_search(word: &str, text: &str) -> Result<Vec<String>, JsValue> {
    word_matches(word, text).map_err(|e| JsValue::from_str(&format!("Regex error : {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_matches_test() {
        let text = "Hello, world! I love the world. Thank you. World...";
        let matches = word_matches("world", text);
        assert_eq!(matches, Ok(vec!["Hello, world!".to_string(), "I love the world.".to_string()]));
    }

    #[test]
    fn word_search_test() {
        let text = r"This is hoge. This is bar. This is foo. But hoge is best. Big Hoge. HOGE. Hoge means hoge.hogehoge.";
        let result = word_search("hoge", text).expect("Failed word_search");
        println!("Result is {:?}", &result);
        assert_eq!(result, vec!["This is hoge.", "But hoge is best.", "Hoge means hoge.", "hogehoge."]);
    }
}
