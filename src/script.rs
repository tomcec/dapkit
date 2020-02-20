use crate::dap;
use json::object;
use std::io::prelude::*;

#[derive(Debug)]
#[derive(PartialEq)]
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
    let mut interaction: Vec<ScriptInteraction> = Vec::new();
    for act in data["interaction"].members() {
        let source: Peers = match act["source"].as_str() {
            Some("ide") => Peers::Ide,
            Some("da") => Peers::Da,
            _ => panic!("source missing"),
        };

        interaction.push(ScriptInteraction {
            source: source,
            content: act["content"].dump(),
        });
    }
    return Ok(DAPScript {
        interactions: interaction,
    });
}

impl DAPScript {
    pub fn run_script(&self, input: &mut dyn Read, output: &mut dyn Write, role: Peers) {
        for step in self.interactions.iter() {
            if step.source == role {
                // Send stuff!
            }
            println!("Step: {:?}", step);
        }
        // 2. Wait for message
        // 3. Match message to expected in script
        // 3.1 If no response found - stop
        let msg: dap::DapMessage = dap::read_message(input).unwrap();
        output
            .write_all(
                json::stringify(object! {
                    "header" => msg.header,
                    "content" => msg.content
                })
                .as_bytes(),
            )
            .unwrap();
    }
}
