use std::convert::Infallible;
use clap::{Parser, parser};

/// Object which contains comma separated list of subjects to promote
#[derive(Debug, Clone)]
pub struct Subjects {
    //TODO: Could also be nice to have an error message when the user inputs 1 or more images 
    //      without a colon in it (indicating they didn't specify the object version)
    //      note this error would only appear if the promotion_type is images
    subjects: Vec<String>
}

impl Subjects {
    fn new() -> Subjects {
        Subjects { subjects: Vec::new() }
    }

    fn push(&mut self, s: String) {
        self.subjects.push(s);
    }
}

impl std::fmt::Display for Subjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_builder = String::new(); 
        // Hacky .clone() shenanigans. Aim to clean up
        for s in self.subjects.clone().into_iter() {
            if string_builder.len() > 0 {
                string_builder.push_str(", ");
            }
            string_builder.push_str(&s.clone());
        }
        write!(f, "Subjects: [{}]", string_builder)
    }
}

impl TryFrom<&str> for Subjects {
    type Error = Infallible; // If not infallible, there is an error I made for this in dead_code

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let seperated_list = s.split(',');
        let mut subjects: Subjects = Subjects::new();
        for item in seperated_list {
            subjects.push(item.to_string());
        }
        Ok(subjects)
    }
}

impl std::str::FromStr for Subjects {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Subjects::try_from(s)
    }
}

/// Contains all information passed to the CLI on initial call
///     Instance: name of instance containing the environments
///     Source: name of the source environment which we want to promote from
///     Destination: name of the destination environment which we want to promote to
///     PromotionType: Type that is getting promoted (object/image)
///     Subjects: comma separated list of objects/images which we want to promote
//TODO: Need to research Clap::parser to see if we can go down an alternate logic path
//      when no args are passed
#[derive(Parser, Debug)]
pub struct CliArguments {
    pub instance: String,
    pub source: String,
    pub destination: String,
    // Temporarily a string, might be worth making this an Enum? (if I can figure it out)
    // Should have `images`, `config maps`, `secrets`, potentially `templates`
    pub promotion_type: String, 
    pub subjects: Subjects
}

impl std::fmt::Display for CliArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "(Instance {}, {}->{}, of type {}, [{}])",
            &self.instance,
            &self.source, &self.destination,
            &self.promotion_type,
            &self.subjects
        )
    }
}


#[test]
fn test_subjects() {
    //Subjects error type is infallible, so this test _should_ be pointless
    let subjects = Subjects::try_from("asdf,fdsa,1234,4321").unwrap();
    //Disgusting test...
    assert_eq!(&"asdf".to_string(), subjects.subjects.get(0).unwrap());
    assert_eq!(&"fdsa".to_string(), subjects.subjects.get(1).unwrap());
    assert_eq!(&"1234".to_string(), subjects.subjects.get(2).unwrap());
    assert_eq!(&"4321".to_string(), subjects.subjects.get(3).unwrap())
}
