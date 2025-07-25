use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DockerContextConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub only_with_files: bool,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for DockerContextConfig<'_> {
    fn default() -> Self {
        Self {
            symbol: "🐳 ",
            style: "blue bold",
            format: "via [$symbol$context]($style) ",
            only_with_files: true,
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec![
                "compose.yml",
                "compose.yaml",
                "docker-compose.yml",
                "docker-compose.yaml",
                "Dockerfile",
            ],
            detect_folders: vec![],
        }
    }
}
