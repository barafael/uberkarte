//! Syntax highlighting for the script editor: a small lexer, no parsing.

const CONTROL: &[&str] = &["on", "function", "end", "then", "pass", "exit", "next", "global", "if", "else", "for", "in", "while", "loop", "do", "until", "return", "break", "continue", "let", "const", "fn", "repeat", "times", "send", "with", "switch", "and", "or", "not", "is"];
const COMMANDS: &[&str] = &["put", "into", "before", "after", "set", "to", "of", "the", "go", "next", "prev", "previous", "first", "last", "answer", "find", "new", "beep", "play", "stop", "hide", "show", "lock", "unlock", "screen"];
const OBJECTS: &[&str] = &["card", "field", "button", "canvas", "picture", "me", "target", "background", "stack"];
const CHUNKS: &[&str] = &["word", "line", "item", "char"];
const VALUES: &[&str] = &["true", "false"];

/// Split a script into (css class, text) runs. Concatenating the texts gives back the script.
pub fn tokens(src: &str) -> Vec<(&'static str, String)> {
    let mut out: Vec<(&'static str, String)> = vec![];
    let mut push = |c: &'static str, s: &str| {
        if let Some(last) = out.last_mut() { if last.0 == c { last.1.push_str(s); return; } }
        out.push((c, s.to_string()));
    };
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut after_on = false;
    while i < chars.len() {
        let c = chars[i];
        let start = i;
        let take = |i: &mut usize, f: &dyn Fn(char) -> bool| { while *i < chars.len() && f(chars[*i]) { *i += 1; } };
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            take(&mut i, &|c| c != '\n');
            push("com", &chars[start..i].iter().collect::<String>());
        } else if c == '/' && chars.get(i + 1) == Some(&'*') {
            i += 2;
            while i < chars.len() && !(chars[i] == '*' && chars.get(i + 1) == Some(&'/')) { i += 1; }
            i = (i + 2).min(chars.len());
            push("com", &chars[start..i].iter().collect::<String>());
        } else if c == '"' || c == '`' {
            i += 1;
            while i < chars.len() && chars[i] != c && chars[i] != '\n' { if chars[i] == '\\' { i += 1; } i += 1; }
            i = (i + 1).min(chars.len());
            push("str", &chars[start..i].iter().collect::<String>());
        } else if c.is_ascii_digit() {
            take(&mut i, &|c| c.is_ascii_alphanumeric() || c == '.' || c == '_');
            push("num", &chars[start..i].iter().collect::<String>());
        } else if c.is_alphabetic() || c == '_' {
            take(&mut i, &|c| c.is_alphanumeric() || c == '_');
            let w: String = chars[start..i].iter().collect();
            let class = if after_on { "msg" }
                else if CONTROL.contains(&w.as_str()) { "kw" }
                else if COMMANDS.contains(&w.as_str()) { "cmd" }
                else if OBJECTS.contains(&w.as_str()) { "obj" }
                else if CHUNKS.contains(&w.as_str()) { "chunk" }
                else if VALUES.contains(&w.as_str()) { "num" }
                else if chars.get(i) == Some(&'(') { "fun" }
                else { "id" };
            after_on = w == "on" && !after_on;
            push(class, &w);
        } else {
            i += 1;
            push(if "{}();,".contains(c) { "punct" } else { "op" }, &c.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn round_trips_and_classifies() {
        let src = "// hi\non mouseUp key {\n    put \"a\" + 1 into field \"x\";\n    rect(1, 2.5, 3, 4, \"red\");\n}\n";
        let t = super::tokens(src);
        assert_eq!(t.iter().map(|(_, s)| s.as_str()).collect::<String>(), src);
        let class_of = |w: &str| t.iter().find(|(_, s)| s.trim() == w).map(|(c, _)| *c);
        assert_eq!(class_of("// hi"), Some("com"));
        assert_eq!(class_of("mouseUp"), Some("msg"));
        assert_eq!(class_of("key"), Some("id"));
        assert_eq!(class_of("put"), Some("cmd"));
        assert_eq!(class_of("field"), Some("obj"));
        assert_eq!(class_of("rect"), Some("fun"));
    }
}
