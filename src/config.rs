use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub root: bool,
    pub charset: String,
    pub end_of_line: String,
    pub insert_final_newline: bool,
    pub trim_trailing_whitespace: bool,
    pub sections: HashMap<String, EditorConfigSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfigSection {
    pub indent_style: String,
    pub indent_size: String,
    pub end_of_line: Option<String>,
    pub charset: Option<String>,
    pub trim_trailing_whitespace: Option<bool>,
    pub insert_final_newline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrettierConfig {
    pub semi: bool,
    pub single_quote: bool,
    pub tab_width: u8,
    pub trailing_comma: String,
    pub print_width: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dev_dependencies: HashMap<String, String>,
}

impl Default for EditorConfig {
    fn default() -> Self {
        let mut sections = HashMap::new();

        // Default section
        sections.insert("*".to_string(), EditorConfigSection {
            indent_style: "space".to_string(),
            indent_size: "2".to_string(),
            end_of_line: Some("lf".to_string()),
            charset: Some("utf-8".to_string()),
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(true),
        });

        // Ruby files
        sections.insert("*.rb".to_string(), EditorConfigSection {
            indent_style: "space".to_string(),
            indent_size: "2".to_string(),
            end_of_line: Some("lf".to_string()),
            charset: Some("utf-8".to_string()),
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(true),
        });

        // JavaScript/TypeScript files
        sections.insert("*.{js,jsx,ts,tsx}".to_string(), EditorConfigSection {
            indent_style: "space".to_string(),
            indent_size: "2".to_string(),
            end_of_line: Some("lf".to_string()),
            charset: Some("utf-8".to_string()),
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(true),
        });

        Self {
            root: true,
            charset: "utf-8".to_string(),
            end_of_line: "lf".to_string(),
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            sections,
        }
    }
}

impl Default for PrettierConfig {
    fn default() -> Self {
        Self {
            semi: true,
            single_quote: true,
            tab_width: 2,
            trailing_comma: "es5".to_string(),
            print_width: 80,
        }
    }
}

impl Default for PackageJson {
    fn default() -> Self {
        let mut dev_dependencies = HashMap::new();
        dev_dependencies.insert("prettier".to_string(), "^3.0.0".to_string());
        dev_dependencies.insert("prettier-plugin-ruby".to_string(), "github:prettier/plugin-ruby".to_string());

        Self {
            name: "project".to_string(),
            version: "0.1.0".to_string(),
            description: "A Ruby project".to_string(),
            dev_dependencies,
        }
    }
}
