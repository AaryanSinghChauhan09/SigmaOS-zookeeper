//! zookeeper model — distributed coordination service.

#[derive(Debug, Clone)]
pub struct ZNode {
    pub path: String,
    pub data: String,
    pub children: Vec<String>,
    pub ephemeral: bool,
    pub sequence: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub znodes: Vec<ZNode>,
    pub next_seq: u64,
}

impl Default for Model {
    fn default() -> Self { Self {
        znodes: vec![
            ZNode { path: "/".into(), data: "".into(), children: vec![], ephemeral: false, sequence: false },
        ],
        next_seq: 1,
    } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, path: &str, data: &str, ephemeral: bool, sequence: bool) -> bool {
        if self.znodes.iter().any(|z| z.path == path) { return false; }
        let final_path = if sequence { format!("{}{:010}", path, self.next_seq) } else { path.to_string() };
        if sequence { self.next_seq += 1; }
        self.znodes.push(ZNode { path: final_path.clone(), data: data.into(), children: vec![], ephemeral, sequence });
        true
    }

    pub fn get(&self, path: &str) -> Option<&str> {
        self.znodes.iter().find(|z| z.path == path).map(|z| z.data.as_str())
    }

    pub fn exists(&self, path: &str) -> bool {
        self.znodes.iter().any(|z| z.path == path)
    }

    pub fn delete(&mut self, path: &str) -> bool {
        if let Some(pos) = self.znodes.iter().position(|z| z.path == path) { self.znodes.remove(pos); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_get_delete() {
        let mut z = Model::new();
        assert!(z.create("/app/config", "value", false, false));
        assert_eq!(z.get("/app/config"), Some("value"));
        assert!(z.delete("/app/config"));
        assert!(!z.exists("/app/config"));
    }
}
