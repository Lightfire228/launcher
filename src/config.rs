use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fs, path::{Path, PathBuf}};
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

        let mut tmp  = String::new();

        let mut i    = 0;


        let result = loop {
            assert!(i < 1000, "Too many variable expansion loops\n(did you accidentally create an expansion loop?)");
            i += 1;

            tmp.clear();
            tmp.push_str(&path);

            let Some(var) = re.captures(&tmp) else {
                break path;
            };

            let var_name = var.get(1).unwrap().as_str();
            let var_str  = var.get_match().as_str();

            let dir = self.get_dir_expect(var_name, proj);

            path = path.replace(&var_str, &dir);
        };

        let result = Path::new(&result).to_owned();

        Some(result)
    }

    fn get_dir(&self, name: &str, proj: Option<&Project>) -> Option<String> {

        if let Some(path) = self.dirs.get(name) {
            return Some(path.to_owned());
        }

        let Some(proj) = proj else { return None; };

        match proj.dirs.get(name) {
            Some(path) => Some(path.to_owned()),
            None       => None
        }
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
