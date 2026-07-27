#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

impl Priority {
    pub fn symbol(&self) -> char {
        match self {
            Priority::Low => '·',
            Priority::Normal => '•',
            Priority::High => '!',
        }
    }

    /// Parse a string into a Priority. Returns Result: Ok on success,
    /// Err with a message on bad input — no panic, the caller decides.
    pub fn parse(input: &str) -> Result<Priority, String> {
        match input {
            "low" => Ok(Priority::Low),
            "normal" => Ok(Priority::Normal),
            "high" => Ok(Priority::High),
            other => Err(format!("unknown priority: {other}")),
        }
    }
}
