pub mod file {
    use std::fs;

    pub fn dir_content(path: &str) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        let _paths = match fs::read_dir("/home".to_string() + path.to_string().as_ref()) {
            Ok(_f) => {
                for path in _f {
                    vec.push(path.unwrap().path().display().to_string());
                }
            }
            Err(_e) => {
                vec.push("Error".parse().unwrap());
                println!("Le dossier est inexistant");
            }

        };
    vec
    }
}

pub mod http {
    pub fn without_cli(string: &str) -> &str {
        string
            .char_indices()
            .next()
            .and_then(|(i, _)| string.get(i + 4..))
            .unwrap_or("")
    }
}