pub fn capitalize(s: &str) -> String {
    let mut string = "".to_string();
    let mut flag = true;

    for ch in s.chars(){
        match ch {
            ' ' => {flag = true; string.push(ch)},
            _ => if flag == true { for c in ch.to_uppercase() {string.push(c)}; flag = false} else { string.push(ch) },
        }
    }
    return string;
}
/*pub fn capitalize (s: &str) ->
String {
    let mut split = s.split(" ");

    for st in split {
        let mut c = st.chars();

        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }
}
    let mut stringa = String::new();
    for c in chars {
        stringa.push(c.to_ascii_uppercase());
    }stringa*/