use crate::str_checker;

// The result of the tokenizer is stored in this struct.
// The 'a lifetime annotation ensures that all the references in the TokensResult
// struct point to data that remains valid for the same lifetime 'a.
// It prevents references from outliving the data they borrow.
pub struct TokensResult<'a> {
    pub is_valid_url: bool,

    pub protocol: Option<&'a str>,
    pub host: Option<&'a str>,
    pub directories: Vec<&'a str>,
    pub filename: Option<&'a str>,
    pub suffix: Option<&'a str>,

    pub words: Vec<&'a str>,
    pub numbers: Vec<&'a str>,
    pub alphanumeric: Vec<&'a str>,
    pub punctuations: Vec<&'a str>,

    pub tokens: Vec<&'a str>,

    // This is for breaking down the tokens further
    // (words, alphabetical, words, punctuations)
    pub granular_tokens: Vec<&'a str>,
}

impl TokensResult<'_> {
    // This function prints the granular breakdown of each token.
    pub fn granular(&self) -> String {
        // Declare the start of the result of the granular breakdown "["
        // murag hold sa Java ni Sir Baste
        let mut granular_string = String::from("[");
        // Declare a variable checking if there are available tokens.
        let mut has_tokens = false;

        // Loop through each granular_token
        for token in &self.granular_tokens {
            // Print the whole token as a full string. (e.g. "abcd")
            granular_string.push_str(format!("\r\n    {} => ", token).as_str());

            // Loop through each character in the string...
            for (index, c) in token.chars().enumerate() {
                // ...then print the character enclosed in single quotes.
                // (e.g. 'a', 'b', 'c', 'd')
                granular_string.push_str(format!("'{}'", c).as_str());
                // Only print a comma when the current character is NOT the last character.
                if index != token.len() - 1 {
                    granular_string.push_str(", ");
                }
            }
            // Set this to true since at least ONE token exists.
            has_tokens = true;
        }
        // Print a newline for proper formatting if there's at least one token.
        if has_tokens {
            granular_string.push_str("\r\n");
        }
        granular_string.push_str("]\r\n");
        granular_string
    }
    pub fn new<'a>(input_str: &'a str) -> TokensResult<'a> {
        // Declare a TokensResult struct with empty content.
        let mut result = TokensResult {
            is_valid_url: false,

            protocol: None,
            host: None,
            directories: Vec::new(),
            filename: None,
            suffix: None,
            numbers: Vec::new(),

            words: Vec::new(),
            alphanumeric: Vec::new(),
            punctuations: Vec::new(),

            // Split the input string into tokens first
            // which is separated by the chosen delimiter
            // (Forwardslash)
            tokens: tokenize(input_str),

            granular_tokens: Vec::new(),
        };

        // Check if it's a URL for a website.
        // The number of tokens should be more than or equal to three
        // [protocol:]/[empty]/[hostname]

        // Also check if it's a valid protocol and make sure to check
        // if the first token (Protocol) is not empty and the
        // second token should be empty.
        result.is_valid_url = result.tokens.len() >= 3
            && is_valid_protocol(result.tokens.get(0).unwrap())
            && !result.tokens.get(0).unwrap().is_empty()
            && result.tokens.get(1).unwrap().is_empty();

        // If it's a valid URL, go through the result tokens
        // and extract the...
        if result.is_valid_url {
            // protocol string from the first token, BUT
            // make sure to remove the separator colon in the end.
            let protocol_str = result.tokens.get(0).unwrap();
            result.protocol = Some(&protocol_str[..protocol_str.len() - 1]);
            result.host = Some(result.tokens.get(2).unwrap());

            // Check if the URL has directories or files after the host.
            // [protocol:]/[empty]/[host]/[filename or directory]
            if result.tokens.len() > 3 {
                // Get a peekable iterator of the tokens 3 onward.
                let it = &mut result.tokens[3..].into_iter().peekable();
                while let Some(token) = it.next() {
                    // Check if it's the last element in the list of tokens.
                    if it.peek().is_none() {
                        // Split the filename and the suffix string in the current token,
                        // then store it in the filename and suffix fields in the result struct.
                        let (filename, suffix) = split_suffix(token);
                        result.filename = Some(filename);
                        result.suffix = Some(suffix);
                    } else {
                        result.directories.push(token);
                    }
                }
            // If the url only contains the protocol and the host,
            // split the suffix string from the host.
            // [protocol:]/[empty]/[host]?suffix=thing
            } else {
                let (host, suffix) = split_suffix(result.tokens.get(2).unwrap());
                result.host = Some(host);
                result.suffix = Some(suffix);
            }
        }

        // Break the tokens further.
        for token in &result.tokens {
            if token.is_empty() {
                continue;
            }
            let mut start_index = 0;
            for (index, c) in token.chars().enumerate() {
                // Split the token if a punctuation is encountered.
                if !c.is_alphanumeric() {
                    // Push the characters before the punctuation and the punctuation itself.
                    let before = &token[start_index..index];
                    if !before.is_empty() {
                        result.granular_tokens.push(before);
                    }
                    result.granular_tokens.push(&token[index..index + 1]);
                    start_index = index + 1;
                }
            }
            let last_part: &str = &token[start_index..];
            if !last_part.is_empty() {
                result.granular_tokens.push(last_part);
            }
        }

        // Add the granular tokens to their respective categories.
        for token in &result.granular_tokens {
            if str_checker::is_number(token) {
                result.numbers.push(token);
            } else if str_checker::is_word(token) {
                result.words.push(token);
            } else if str_checker::is_alphanumeric(token) {
                result.alphanumeric.push(token);
            // Discard empty tokens.
            } else if !token.is_empty() {
                result.punctuations.push(token);
            }
        }

        result
    }
}

pub fn is_valid_protocol(protocol_str: &str) -> bool {
    // Skip empty protocol string.
    if protocol_str.len() == 0 {
        return false;
    }

    // A valid protocol should end with one colon and it should contain no other symbols.
    let end_index: usize = protocol_str.len() - 1;
    for (current_index, c) in protocol_str.chars().enumerate() {
        if (current_index != end_index && !c.is_alphabetic())
            || (current_index == end_index && c != ':')
        {
            return false;
        }
    }
    true
}

// Some URLs have an extra part after the filename.
// --> for example: https://www.example.com/article/123?category=technology#section2
// Separation should be done to properly extract the filename.
pub fn split_suffix(mixed: &str) -> (&str, &str) {
    let first_part: &str;
    let suffix: &str;

    for (index, c) in mixed.chars().enumerate() {
        if !c.is_alphanumeric() && (c == '?' || c == '#') {
            // Split the first part (filename or hostname) up to the index.
            first_part = &mixed[..index];
            // Then, split the second part (the URL suffix)
            // from the character after the index until the end of the string.
            suffix = &mixed[index + 1..];
            return (first_part, suffix);
        }
    }
    (mixed, "")
}

// Split the input string into different string slices
// based on the delimeter.
fn tokenize(input_str: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut start_index: usize = 0;
    for (index, c) in input_str.chars().enumerate() {
        if c == '/' {
            // If a forward slash is encountered, a string slice
            // of the input string is added to the list of tokens
            // starting from the start_index until the current position
            // where the forward slash is encountered.
            let extracted: &str = &input_str[start_index..index];
            tokens.push(extracted);
            // Then, update the start_index to the next character after the forward slash.
            start_index = index + 1;
        }
    }
    // Add the rest of the unprocessed string to the list of tokens.
    let last_part: &str = &input_str[start_index..];
    tokens.push(last_part);
    tokens
}
