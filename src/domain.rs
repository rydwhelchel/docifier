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

///Iterator wrapper which stores the index state of our iterator
/// Without this wrapper, we would have to store inedex in Targets... which is just wrong
pub struct TargetsIterator <'a> {
    targets: &'a Targets,
    index: usize,
}

impl<'a> Iterator for TargetsIterator<'a> {
    //This is a pattern I really will need to get familiar with
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.targets.targets.len() {
            let result = Some(&self.targets.targets[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl Targets {
    fn new() -> Targets {
        Targets { targets: Vec::new() }
    }

    fn push(&mut self, s: String) {
        self.targets.push(s);
    }

    //Awesome! I like this pattern now :)
    pub fn iter(&self) -> TargetsIterator {
        TargetsIterator {
            targets: self,
            index: 0,
        }
    }

    // Special logic to ensure that when user input is just the return key (nothing entered)
    // then it is treated as a 0 length vec, instead of 1 length with a "" as the value
    pub fn len(&self) -> usize {
        if self.targets.len() == 1 {
            if self.targets.get(0).unwrap().len() > 0 {
                return self.targets.len();
            }
            return 0;
        }
        self.targets.len()
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
    Line(PromotionBatch)
}

#[derive(Parser)]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Commands,
}

/// Contains all information needed to document a batch
///     Instance: name of instance containing the environments
///     Source: name of the source environment which we want to promote from
///     Destination: name of the destination environment which we want to promote to
///     PromotionType: Type that is getting promoted (object/image)
///     Targets: comma separated list of objects/images which we want to promote
#[derive(Parser, Debug)]
pub struct PromotionBatch {
    // #[arg(short, long)]
    pub instance: String,

    // #[arg(short, long)]
    pub source: String,

    // #[arg(short, long)]
    pub destination: String,

    // Temporarily a string, might be worth making this an Enum? (if I can figure it out)
    // Should have `images`, `config-maps`, `secrets`, potentially `templates`
    // #[arg(short, long)]
    pub promotion_type: String, 

    // #[arg(short, long)]
    pub targets: Targets
}

impl std::fmt::Display for PromotionBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "(Instance: {}, Path: {}->{}, Promotion Type: {}, {})",
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
