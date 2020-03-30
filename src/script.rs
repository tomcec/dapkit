use crate::dap;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Peers {
    Ide,
    Da,
}

#[derive(Debug, Clone)]
pub struct ScriptInteraction {
    pub source: Peers,
    pub content: String,
}

#[derive(Debug)]
pub struct DAPScript {
    pub interactions: Vec<ScriptInteraction>,
}

impl std::convert::From<&DAPScript> for json::JsonValue {
    fn from(script: &DAPScript) -> json::JsonValue {
        json::object!(
            interactions: script.interactions.clone(),
        )
    }
}

impl std::convert::From<ScriptInteraction> for json::JsonValue {
    fn from(si: ScriptInteraction) -> json::JsonValue {
        json::object!(
            source: si.source,
            content: si.content.clone(),
        )
    }
}

impl std::convert::From<Peers> for json::JsonValue {
    fn from(pear: Peers) -> json::JsonValue {
        match pear {
            Peers::Ide => json::JsonValue::String(String::from("ide")),
            Peers::Da => json::JsonValue::String(String::from("da")),
        }
    }
}

pub fn load_script(filename: &str) -> Result<DAPScript, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let data = json::parse(&content).unwrap();
    let mut interaction: Vec<ScriptInteraction> = Vec::new();
    for act in data["interactions"].members() {
        let source: Peers = match act["source"].as_str() {
            Some("ide") => Peers::Ide,
            Some("da") => Peers::Da,
            _ => panic!("source missing"),
        };

        
        interaction.push(ScriptInteraction {
            source: source,
            content: match &act["content"] {
                json::JsonValue::String(text) => String::from(text),
                other => other.dump(),
            },
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
                dap::send_message(output, &step.content).unwrap();
                println!("send: {}", &step.content);
            } else {
                let msg: dap::DapMessage = dap::read_message(input).unwrap();
                println!("resv: {}", &msg.content);
                DAPScript::match_message(&msg.content, &step.content);
                // Some magic base on match result
            }
        }
    }

    // TODO
    fn match_message(_expected: &String, _actual: &String) -> bool {
        // let expected = json::parse(&expected).unwrap();
        // let actual = json::parse(&actual).unwrap();
        // TODO compare logic here
        // expected.eq(&actual)
        return true;
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
