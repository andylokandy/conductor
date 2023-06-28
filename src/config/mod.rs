use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};

#[derive(Deserialize, Debug, Clone)]
pub struct ConductorConfig {
    pub server: ServerConfig,
    pub logger: LoggerConfig,
    pub sources: Vec<SourceDefinition>,
    pub endpoints: Vec<EndpointDefinition>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EndpointDefinition {
    pub path: String,
    pub from: String,
    #[serde(default = "default_endpoint_graphiql")]
    pub graphiql: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Level(pub(super) tracing::Level);

impl Level {
    pub fn into_level(self) -> tracing::Level {
        self.0
    }
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr as _;

        let s = String::deserialize(deserializer)?;
        tracing::Level::from_str(&s)
            .map(Level)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggerConfig {
    #[serde(default = "default_logger_level")]
    pub level: Level,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_server_port")]
    port: u16,
    #[serde(default = "default_server_host")]
    host: String,
}

fn default_endpoint_graphiql() -> bool {
    true
}
fn default_logger_level() -> Level {
    Level(tracing::Level::INFO)
}
fn default_server_port() -> u16 {
    9000
}
fn default_server_host() -> String {
    "127.0.0.1".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SourceDefinition {
    #[serde(rename = "graphql")]
    GraphQL {
        id: String,
        config: GraphQLSourceConfig,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphQLSourceConfig {
    pub endpoint: String,
}

#[tracing::instrument]
pub async fn load_config(file_path: &String) -> ConductorConfig {
    let path = Path::new(file_path);
    let contents = read_to_string(file_path).expect("Failed to read config file");

    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("json") => serde_json::from_str::<ConductorConfig>(&contents)
                .expect("Failed to parse config file"),
            Some("yaml") | Some("yml") => serde_yaml::from_str::<ConductorConfig>(&contents)
                .expect("Failed to parse config file"),
            _ => panic!("Unsupported config file extension"),
        },
        None => panic!("Config file has no extension"),
    }
}
