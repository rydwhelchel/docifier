use clap::{Parser, Subcommand};
use std::convert::Infallible;
use serde::Deserialize;

/// Object which contains comma separated list of targets to promote
#[derive(Debug, Clone, PartialEq)]
pub struct Targets {
    targets: Vec<String>,
}

///Iterator wrapper which stores the index state of our iterator
/// Without this wrapper, we would have to store index in Targets
pub struct TargetsIterator<'a> {
    targets: &'a Targets,
    index: usize,
}

impl<'a> Iterator for TargetsIterator<'a> {
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
        Targets {
            targets: Vec::new(),
        }
    }

    fn push(&mut self, s: String) {
        self.targets.push(s);
    }

    pub fn iter(&self) -> TargetsIterator {
        TargetsIterator {
            targets: self,
            index: 0,
        }
    }

    //treat empty string as a non element (zero length)
    pub fn len(&self) -> usize {
        if self.targets.len() <= 1 {
            if !self.targets.get(0).unwrap().is_empty() {
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
        for s in self.targets.clone().into_iter() {
            if !string_builder.is_empty() {
                string_builder.push(',');
            }
            string_builder.push_str(&s.clone());
        }
        write!(f, "{}", string_builder)
    }
}

impl TryFrom<&str> for Targets {
    type Error = Infallible;

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

impl Targets {
    pub fn prepare(&mut self) -> Vec<Targets> {
        let mut splits = self.targets.len() / 3;
        if self.targets.len() % 3 > 0 {
            splits += 1;
        }
        
        let mut chunks: Vec<Targets> = Vec::new();

        //TODO: Clean up this logic
        while splits > 0 {
            let mut count = 3;
            let mut temp_vec = Vec::new();
            while count > 0 {
                temp_vec.push(self.targets.remove(0));
                if self.targets.is_empty() {
                    break;
                }
                count -= 1;
            }
            let temp_targets = Targets { targets: temp_vec };
            chunks.push(temp_targets);
            splits -= 1;
        }

        chunks
    }
}

#[derive(Parser)]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New(PromotionBatch),
    Add(PromotionBatch)
}

/// Contains all information needed to document a batch
#[derive(Parser, Debug, Clone)]
pub struct PromotionBatch {
    //TODO: Correct this object for single line parsing
    /// Name of instance containing the environments
    pub instance: String,

    /// Name of the source environment which we want to promote from
    pub source: String,

    /// Name of the destination environment which we want to promote to
    pub destination: String,

    //TODO: Need to give user hints on what to pass for this
    // Temporarily a string, might be worth making this an Enum?
    // Should have `images`, `config-maps`, `secrets`, potentially `templates`
    /// Type that is getting promoted ('images', 'config-maps', 'secrets', 'templates')
    pub promotion_type: String,

    /// Comma separated list of objects/images which we want to promote
    pub targets: Targets,
}

impl std::fmt::Display for PromotionBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Instance: {}, Path: {}->{}, Promotion Type: {}, {})",
            &self.instance, &self.source, &self.destination, &self.promotion_type, &self.targets
        )
    }
}

pub enum LineType {
    Instance(String),
    Path(String, String),
    PromoteImages(Vec<String>),
    PromoteConfigMaps(Vec<String>),
    PromoteSecrets(Vec<String>),
    PromoteTemplates(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct FormatLines {
    pub instance: String,
    pub instance_subline: String,
    pub path: String,
    pub promote_images: String,
    pub promote_config_maps: String,
    pub promote_secrets: String,
    pub promote_templates: String
}

impl FormatLines {
    pub fn comparable(&self) -> ComparableFormatLines {
        let instance = self.instance.split(' ').map(|x| x.to_string()).collect();
        let path = self.path.split(' ').map(|x| x.to_string()).collect();
        let promote_images = self.promote_images.split(' ').map(|x| x.to_string()).collect();
        let promote_config_maps = self.promote_config_maps.split(' ').map(|x| x.to_string()).collect();
        let promote_secrets = self.promote_secrets.split(' ').map(|x| x.to_string()).collect();
        let promote_templates = self.promote_templates.split(' ').map(|x| x.to_string()).collect();

        ComparableFormatLines {
            instance,
            path,
            promote_images,
            promote_config_maps,
            promote_secrets,
            promote_templates
        }
    }

}

#[derive(Debug)]
pub struct ComparableFormatLines {
    pub instance: Vec<String>,
    pub path: Vec<String>,
    pub promote_images: Vec<String>,
    pub promote_config_maps: Vec<String>,
    pub promote_secrets: Vec<String>,
    pub promote_templates: Vec<String>
}

impl ComparableFormatLines {
    //todo: write test cases
    pub fn evaluate(&self, line: String) -> Result<LineType, &str> {
        let line: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();
        
        // Test Instance
        let value = compare_lines(&self.instance, &line);
        if value != None {
            let value = value.unwrap();
            assert!(value.len() == 1, "Incorrect length for values extracted from instance line");
            return Ok(LineType::Instance(value.get(0).unwrap().to_string()));
        }

        // Test Path 
        let value = compare_lines(&self.path, &line);
        if value != None {
            let value = value.unwrap();
            assert!(value.len() == 2, "Incorrect length for values extracted from instance line");
            return Ok(LineType::Path(value.clone().get(0).unwrap().to_string(),
                value.get(1).unwrap().to_string()));
        }

        // Test Images
        let value = compare_lines(&self.promote_images, &line);
        if value != None {
            return Ok(LineType::PromoteImages(value.unwrap()))
        }

        // Test Config Maps
        let value = compare_lines(&self.promote_config_maps, &line);
        if value != None {
            return Ok(LineType::PromoteConfigMaps(value.unwrap()))
        }

        // Test Secrets
        let value = compare_lines(&self.promote_secrets, &line);
        if value != None {
            return Ok(LineType::PromoteSecrets(value.unwrap()))
        }

        // Test Templates
        let value = compare_lines(&self.promote_templates, &line);
        if value != None {
            return Ok(LineType::PromoteTemplates(value.unwrap()))
        }
        return Err("Did not match")
    }

}

fn compare_lines(format_line: &Vec<String>, line: &Vec<String>) -> Option<Vec<String>> {
    let mut value = vec![];
    for (i, elem) in format_line.iter().enumerate() {
        if i >= line.len() {
            return None
        } else if elem.contains('{') {
            value.push(line.get(i).unwrap().to_string())
        } else if elem != line.get(i).unwrap() {
            return None
        }
    }
    return Some(value)
}

#[test]
fn test_targets() {
    //Targets error type is infallible, so this test _should_ be pointless
    let targets: Vec<String> = Targets::try_from("asdf,fdsa,1234,4321").unwrap().targets;
    assert_eq!(&"asdf".to_string(), targets.get(0).unwrap());
    assert_eq!(&"fdsa".to_string(), targets.get(1).unwrap());
    assert_eq!(&"1234".to_string(), targets.get(2).unwrap());
    assert_eq!(&"4321".to_string(), targets.get(3).unwrap())
}

#[test]
fn test_targets_prepare() {
    let targets = Targets::try_from("1,2,3,4,5").unwrap();
    let prepped = targets.clone().prepare();
    assert_eq!(prepped, vec![Targets { targets: vec!["1".to_string(),"2".to_string(),"3".to_string()]},Targets { targets: vec!["4".to_string(),"5".to_string()]}])
}

#[test]
fn test_targets_prepare_exact_three() {
    let targets = Targets::try_from("1,2,3").unwrap();
    let prepped = targets.clone().prepare();
    assert_eq!(prepped, vec![Targets { targets: vec!["1".to_string(),"2".to_string(),"3".to_string()]}])
}

#[test]
fn test_targets_prepare_long_list() {
    let targets = Targets::try_from("1,2,3,4,5,6,7,8,9").unwrap();
    let prepped = targets.clone().prepare();
    assert_eq!(prepped, vec![Targets { targets: vec!["1".to_string(),"2".to_string(),"3".to_string()]},Targets { targets: vec!["4".to_string(),"5".to_string(),"6".to_string()]},Targets { targets: vec!["7".to_string(),"8".to_string(),"9".to_string()]}])
}

#[test]
fn test_targets_prepare_long_list_plus_one() {
    let targets = Targets::try_from("1,2,3,4,5,6,7,8,9,10").unwrap();
    let prepped = targets.clone().prepare();
    assert_eq!(prepped, vec![Targets { targets: vec!["1".to_string(),"2".to_string(),"3".to_string()]},Targets { targets: vec!["4".to_string(),"5".to_string(),"6".to_string()]},Targets { targets: vec!["7".to_string(),"8".to_string(),"9".to_string()]},Targets { targets: vec!["10".to_string()]}])
}
