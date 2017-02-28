#[derive(Debug)]
pub struct Machine {
    id: String,
    name: String,
    provider: String,
    state: String,
    directory: String
}

impl Machine {
    pub fn from_output_line(line: &str) -> Machine {
        let line = line.to_string();
        let words:Vec<&str> = line.split_whitespace().collect();

        Machine{
            id: words[0].to_string(),
            name: words[1].to_string(),
            provider: words[2].to_string(),
            state: words[3].to_string(),
            directory: words[4].to_string()
        }
    }
}
