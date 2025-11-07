use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}, fs};
use regex::Regex;
// use std::fs;

type Dirs = HashMap<String, String>;

pub fn get_config() -> Config {

    let config_str = fs::read_to_string("./config.yaml").expect("Unable to read config file");

    let config: ConfigYaml = serde_yml::from_str(&config_str).expect("Unable to parse config file");

    config.into()
}

#[derive(Debug)]
pub struct Config {
    pub projects: Vec<Project>
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub code: String,

    pub vscode:   Vec<String>,
    pub zed:      Vec<String>,
    pub obsidian: String,
    pub terminal: Vec<String>,
}




#[derive(Serialize, Deserialize, Debug)]
struct ConfigYaml {
    #[serde(default)] pub projects: Vec<ProjectYaml>,
    #[serde(default)] pub dirs:     Dirs,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectYaml {
    pub name: String,
    pub code: String,

    #[serde(default)] pub vscode:   Vec<String>,
    #[serde(default)] pub zed:      Vec<String>,
    #[serde(default)] pub obsidian: String,
    #[serde(default)] pub terminal: Vec<String>,
    // #[serde(default)] pub terminal: TerminalYaml,

    #[serde(default)] pub dirs:     Dirs,
}



// #[derive(Serialize, Deserialize, Default)]
// pub struct TerminalYaml {
//     #[serde(default)] pub tabs: Vec<String>,
// }



impl From<ConfigYaml> for Config {

    fn from(value: ConfigYaml) -> Self {
        Self {
            projects: value.projects.iter().map(|p| Project::from_yaml(&p, &value)).collect()
        }
    }
}

impl Project {

    fn from_yaml(yaml: &ProjectYaml, config: &ConfigYaml) -> Self {

        let into = |iter: &[String]| { iter
            .iter()
            .map(|dir|
                config.expand_dir(&dir, Some(&yaml))
            )
            .collect()
        };

        Self {
            vscode:   into(&yaml.vscode),
            zed:      into(&yaml.zed),
            terminal: into(&yaml.terminal),
            obsidian: yaml.obsidian.to_owned(),

            name: yaml.name.to_owned(),
            code: yaml.code.to_owned(),
        }
    }
}


impl ConfigYaml {


    pub fn expand_dir(&self, name: &str, proj: Option<&ProjectYaml>) -> String {

        let     re   = Regex::new(r"\{(\w+)\}").unwrap();
        let mut path = name.to_owned();

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

        result
    }

    fn get_dir(&self, name: &str, proj: Option<&ProjectYaml>) -> Option<String> {
        self.dirs.get(name)
            .cloned()
            .or_else(||
                proj.and_then(|p| p.dirs.get(name).cloned())
            )
    }

    fn get_dir_expect(&self, name: &str, proj: Option<&ProjectYaml>) -> String {
        self.get_dir(name, proj).unwrap_or_else(|| panic!("Unable to find dir definition for {name}"))
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = get_config();

        // TODO: create a config.test.yaml
        assert_eq!(&config.projects[0].code, "tk")
    }

}
