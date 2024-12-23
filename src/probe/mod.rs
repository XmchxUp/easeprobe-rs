pub trait Prober {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
}

pub struct ProbeResult {
    pub name: String,
}
