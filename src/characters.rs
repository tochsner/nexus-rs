pub fn is_punctuation(character: &char) -> bool {
    *character == '('
        || *character == ')'
        || *character == '['
        || *character == ']'
        || *character == '{'
        || *character == '}'
        || *character == '/'
        || *character == '\\'
        || *character == '/'
        || *character == ';'
        || *character == ':'
        || *character == '='
        || *character == '*'
        || *character == '\''
        || *character == '"'
        || *character == 'v'
        || *character == '+'
        || *character == '-'
        || *character == '<'
        || *character == '>'
}

pub fn is_whitespace(character: &char) -> bool {
    return *character == ' ' || *character == '\t' || *character == '\n';
}

pub fn is_character(character: &char) -> bool {
    !is_whitespace(character) && !is_punctuation(character)
}
