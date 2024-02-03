use clap::Parser;
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
        self.targets.push(s.trim().to_string());
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> TargetsIterator {
        TargetsIterator {
            targets: self,
            index: 0,
        }
    }

    //treat empty string as a non element (zero length)
    #[allow(dead_code)]
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

/// Pass in the following arguments to create a chunk of documentation for the passed in arguments
#[derive(Parser, Debug, Clone)]
pub struct PromotionBatch {
    /// Name of instance containing the environments
    pub instance: String,

    /// Name of the source environment which we want to promote from
    pub source: String,

    /// Name of the destination environment which we want to promote to
    pub destination: String,

    /// Type that is getting promoted ('images', 'config-maps', 'secrets')
    pub promotion_type: String,

    /// Comma separated list of objects/images which we want to promote, wrap in quotes to allow
    /// spaces
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

#[derive(Debug, Deserialize)]
pub struct FormatLines {
    pub instance: String,
    pub path: String,
    pub promote_images: String,
    pub promote_config_maps: String,
    pub promote_secrets: String,
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
