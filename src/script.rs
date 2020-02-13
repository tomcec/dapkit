#[derive(Debug)]
pub enum Peers {
    Ide,
    Da,
}

#[derive(Debug)]
pub struct ScriptInteraction {
    pub source: Peers,
    pub content: String,
}

#[derive(Debug)]
pub struct DAPScript {
    pub interactions: Vec<ScriptInteraction>,
}

pub fn load_script(filename: &str) -> Result<DAPScript, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let data = json::parse(&content).unwrap();
    println!("Interaction in {}\n{}", filename, data["interaction"]);
    let mut interaction: Vec<ScriptInteraction> = Vec::new();
    for act in data["interaction"].members() {
        let source: Peers = match act["source"].as_str() {
            Some("ide") => Peers::Ide,
            Some("da") => Peers::Da,
            _ => panic!("source missing")
        };

        interaction.push(ScriptInteraction {
            source: source,
            content: act["content"].dump()
        });
    }
    return Ok(DAPScript {
        interactions: interaction,
    });
}