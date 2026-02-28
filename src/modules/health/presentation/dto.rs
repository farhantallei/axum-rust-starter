use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Cpu {
    pub model: String,
    pub usage: f32,
}

#[derive(Debug, Serialize)]
pub struct GetHealthResponse {
    pub status: String,
    pub db: String,
    pub timestamp: String,
    pub uptime: u64,
    pub version: String,
    pub platform: String,
    pub arch: String,
    pub pid: String,
    pub cpus: Vec<Cpu>,
    pub memory: String,
}
