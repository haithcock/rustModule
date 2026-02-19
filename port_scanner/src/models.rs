#[derive(Debug, Clone)]
pub struct PortFinding {
    pub port: u16,
    pub banner: Option<String>,
}
