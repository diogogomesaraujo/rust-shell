pub fn red_text(s: String) -> String {
    let mut aux: String = String::from("\u{001b}[31m");
    aux.push_str(&s);
    aux.push_str("\u{001b}[0m");

    aux
}

pub fn teal_text(s: String) -> String {
    let mut aux: String = String::from("\u{001b}[36m");
    aux.push_str(&s);
    aux.push_str("\u{001b}[0m");

    aux
}
