struct NATOEncryptor {
    mapping: Vec<(&'static str, &'static str)>,
}

impl NATOEncryptor {
    fn new() -> Self {
        Self {
            mapping: vec![
                ("A", "Alpha"),
                ("B", "Bravo"),
                ("C", "Charlie"),
                ("D", "Delta"),
                ("E", "Echo"),
                ("F", "Foxtrot"),
                ("G", "Golf"),
                ("H", "Hotel"),
                ("I", "India"),
                ("J", "Juliet"),
                ("K", "Kilo"),
                ("L", "Lima"),
                ("M", "Mike"),
                ("N", "November"),
                ("O", "Oscar"),
                ("P", "Papa"),
                ("Q", "Quebec"),
                ("R", "Romeo"),
                ("S", "Sierra"),
                ("T", "Tango"),
                ("U", "Uniform"),
                ("V", "Victor"),
                ("W", "Whiskey"),
                ("X", "Xray"),
                ("Y", "Yankee"),
                ("Z", "Zulu"),
            ],
        }
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let mut ciphertext = String::new();
        for c in plaintext.chars() {
            let c = c.to_ascii_uppercase();
            let code = match self.mapping.iter().find(|&&(key, _)| key == &c.to_string()) {
                Some(&(_, value)) => value.to_string(),
                None => c.to_string(),
            };
            ciphertext.push_str(&format!("{} ", code));
        }
        ciphertext.trim().to_string()
    }
}

fn main() {
    let encryptor = NATOEncryptor::new();
    let plaintext = "Hello, world!";
    let ciphertext = encryptor.encrypt(plaintext);
    println!("{}", ciphertext);
}
