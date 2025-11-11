use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet}, fs, path::Path};
use regex::Regex;

type Dirs = HashMap<String, String>;

pub fn get_config() -> Config {
    let path = local_config_path();
    get_config_from(&path)
}

pub fn get_config_from(path: &str) -> Config {

    check_config_file(path);

    let config_str = fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read config file: {path}"));

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
    pub obsidian: Option<String>,
    pub terminal: Terminal,
}

#[derive(Debug)]
pub struct Terminal {
    pub tabs: Vec<String>
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
    #[serde(default)] pub obsidian: Option<String>,
    // #[serde(default)] pub terminal: TerminalYaml,
    #[serde(default)] pub terminal: Vec<String>,

    #[serde(default)] pub dirs:     Dirs,
}

// #[derive(Serialize, Deserialize, Default, Debug)]
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

        Self {
            vscode:   expand_all(&yaml.vscode,   config, yaml),
            zed:      expand_all(&yaml.zed,      config, yaml),

            terminal: Terminal::from_yaml(&yaml, &config),
            obsidian: yaml.obsidian.clone(),

            name: yaml.name.to_owned(),
            code: yaml.code.to_owned(),
        }
    }
}

impl Terminal {
    fn from_yaml(yaml: &ProjectYaml, config: &ConfigYaml) -> Self {
        Self {
            tabs: expand_all(&yaml.terminal, config, yaml)
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

fn expand_all(
    iter:   &[String],
    config: &ConfigYaml,
    proj:   &ProjectYaml
)
    -> Vec<String>
{ iter
    .iter()
    .map(|dir|
        config.expand_dir(&dir, Some(proj))
    )
    .collect()
}

fn local_config_path() -> String {
    format!("{}/.config/fourth_bridge/config.yaml", env!("HOME"))
}

fn check_config_file(path: &str) {
    if !Path::new(path).exists() {
        panic!("Config file does not exist: {path}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = get_config_from("./config.test.yaml");

        assert_eq!(&config.projects[0].code, "p1")
    }

}
