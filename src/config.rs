use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}};
use regex::Regex;
// use std::fs;

type Dirs = HashMap<String, String>;

pub fn get_config() -> Config {

    let config_str = fs::read_to_string("./config.yaml").expect("Unable to read config file");

    let config: Config = serde_yml::from_str(&config_str).expect("Unable to parse config file");

    config
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)] pub projects: Vec<Project>,
    #[serde(default)] pub dirs:     Dirs,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub code: String,

    #[serde(default)] pub vscode:   Vec<String>,
    #[serde(default)] pub zed:      Vec<String>,
    #[serde(default)] pub obsidian: String,
    #[serde(default)] pub terminal: Vec<String>,
    // #[serde(default)] pub terminal: Terminal,

    #[serde(default)] pub dirs:     Dirs,
}



// #[derive(Serialize, Deserialize, Default)]
// pub struct Terminal {
//     #[serde(default)] pub tabs: Vec<String>,
// }


impl Config {


    pub fn expand_dir(&self, name: &str, proj: Option<&Project>) -> Option<PathBuf> {

        let     re   = Regex::new(r"\{(\w+)\}").unwrap();
        let mut path = self.get_dir_expect(name, proj);

        let mut i    = 0;

        let mut map = HashSet::new();


        let result = loop {
            assert!(i < 1000, "Too many variable expansion loops");
            i += 1;

            let Some(var) = re.captures(&path) else {
                break path;
            };


            let var_str  = var.get(0).unwrap().as_str();
            let var_name = var.get(1).unwrap().as_str().to_owned();

            if map.contains(&var_name) {
                panic!("Variable expansion cycle detected: {var_str}");
            }


            let dir = self.get_dir_expect(&var_name, proj);

            path = path.replace(var_str, &dir);

            map.insert(var_name.to_owned());

        };

        let result = Path::new(&result).to_owned();

        Some(result)
    }

    fn get_dir(&self, name: &str, proj: Option<&Project>) -> Option<String> {
        self.dirs.get(name)
            .cloned()
            .or_else(||
                proj.and_then(|p| p.dirs.get(name).cloned())
            )
    }

    fn get_dir_expect(&self, name: &str, proj: Option<&Project>) -> String {
        self.get_dir(name, proj).unwrap_or_else(|| panic!("Unable to find dir definition for {name}"))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = get_config();

        assert_eq!(&config.projects[0].code, "tk")
    }

}
