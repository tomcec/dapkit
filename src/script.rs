use crate::dap;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
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
                output.write_all(step.content.as_bytes()).unwrap();
            } else {
                let msg: dap::DapMessage = dap::read_message(input).unwrap();
                DAPScript::match_message(&msg.content, &step.content);
                // Some magic base on match result
            }
        }
    }

    fn match_message(expected: &String, actual: &String) -> bool {
        let expected = json::parse(&expected).unwrap();
        let actual = json::parse(&actual).unwrap();
        // TODO compare logic here
        expected.eq(&actual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_message_test() {
        let result = DAPScript::match_message(&String::from("{}"), &String::from("{}"));
        assert_eq!(result, true);
    }
}
