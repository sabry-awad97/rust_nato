use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Language {
    English,
}

struct NATO {
    mappings: HashMap<Language, Vec<(&'static str, &'static str)>>,
    language: Language,
}

impl NATO {
    fn new(language: Language) -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(
            Language::English,
            vec![
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
        );
        Self { mappings, language }
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let mapping = self.mappings.get(&self.language).unwrap();
        let mut ciphertext = String::new();
        for c in plaintext.chars() {
            let c = c.to_ascii_uppercase();
            let code = match mapping.iter().find(|&&(key, _)| key == &c.to_string()) {
                Some(&(_, value)) => value.to_string(),
                None => c.to_string(),
            };
            ciphertext.push_str(&format!("{} ", code));
        }
        ciphertext.trim().to_string()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let mapping = self.mappings.get(&self.language).unwrap();
        let mut plaintext = String::new();
        let words: Vec<&str> = ciphertext.split(' ').collect();
        for (i, word) in words.iter().enumerate() {
            let letter = match mapping.iter().find(|&&(_, value)| value == *word) {
                Some(&(key, _)) => key,
                None => word,
            };
            plaintext.push_str(&letter);
            if i < words.len() - 1 {
                plaintext.push(' ');
            }
        }
        plaintext
    }
}

fn main() {
    let nato = NATO::new(Language::English);
    let plaintext = "Hello, world!";
    let ciphertext = nato.encrypt(plaintext);
    println!("{}", ciphertext);

    let plaintext = nato.decrypt(&ciphertext);
    println!("{}", plaintext);
}
