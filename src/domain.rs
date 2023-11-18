use std::convert::Infallible;
use clap::{Parser, parser, Subcommand};

/// Object which contains comma separated list of targets to promote
#[derive(Debug, Clone)]
pub struct Targets {
    //TODO: Could also be nice to have an error message when the user inputs 1 or more images 
    //      without a colon in it (indicating they didn't specify the object version)
    //      note this error would only appear if the promotion_type is images
    targets: Vec<String>
}

impl Targets {
    fn new() -> Targets {
        Targets { targets: Vec::new() }
    }

    fn push(&mut self, s: String) {
        self.targets.push(s);
    }
}

impl std::fmt::Display for Targets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_builder = String::new(); 
        // Hacky .clone() shenanigans. Aim to clean up
        for s in self.targets.clone().into_iter() {
            if string_builder.len() > 0 {
                string_builder.push_str(", ");
            }
            string_builder.push_str(&s.clone());
        }
        write!(f, "Targets: [{}]", string_builder)
    }
}

impl TryFrom<&str> for Targets {
    type Error = Infallible; // If not infallible, there is an error I made for this in dead_code

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let seperated_list = s.split(',');
        let mut targets: Targets = Targets::new();
        for item in seperated_list {
            targets.push(item.to_string());
        }
        Ok(targets)
    }
}

impl std::str::FromStr for Targets {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Targets::try_from(s)
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Prompt,
    Line(OneLineArguments)
}

#[derive(Parser)]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Commands,
}

/// Contains all information passed to the CLI on initial call
///     Instance: name of instance containing the environments
///     Source: name of the source environment which we want to promote from
///     Destination: name of the destination environment which we want to promote to
///     PromotionType: Type that is getting promoted (object/image)
///     Targets: comma separated list of objects/images which we want to promote
#[derive(Parser, Debug)]
pub struct OneLineArguments {
    #[arg(short, long)]
    pub instance: String,

    #[arg(short, long)]
    pub source: String,

    #[arg(short, long)]
    pub destination: String,

    // Temporarily a string, might be worth making this an Enum? (if I can figure it out)
    // Should have `images`, `config-maps`, `secrets`, potentially `templates`
    #[arg(short, long)]
    pub promotion_type: String, 

    #[arg(short, long)]
    pub targets: Targets
}

impl std::fmt::Display for OneLineArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "(Instance {}, {}->{}, of type {}, [{}])",
            &self.instance,
            &self.source, &self.destination,
            &self.promotion_type,
            &self.targets
        )
    }
}


#[test]
fn test_targets() {
    //Targets error type is infallible, so this test _should_ be pointless
    let targets = Targets::try_from("asdf,fdsa,1234,4321").unwrap().targets;
    //Disgusting test...
    assert_eq!(&"asdf".to_string(), targets.get(0).unwrap());
    assert_eq!(&"fdsa".to_string(), targets.get(1).unwrap());
    assert_eq!(&"1234".to_string(), targets.get(2).unwrap());
    assert_eq!(&"4321".to_string(), targets.get(3).unwrap())
}
