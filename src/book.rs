use crate::note::{Note, Priority};
use std::fs;

#[derive(Default)]
pub struct NoteBook {
    pub notes: Vec<Note>,
    next_id: u32,
}

impl NoteBook {
    pub fn add(&mut self, text: String, priority: Priority) {
        self.next_id += 1;
        self.notes.push(Note::new(self.next_id, text, priority));
    }

    pub fn list(&self) {
        for note in &self.notes {
            println!("{}", note.summary());
        }
    }

    pub fn count(&self) -> usize {
        self.notes.len()
    }

    /// Serialize each note to a "priority|text" line and write the file.
    pub fn save(&self, path: &str) -> Result<(), String> {
        let mut out = String::new();
        for note in &self.notes {
            let pri = format!("{:?}", note.priority).to_lowercase();
            out.push_str(&format!("{pri}|{}\n", note.text));
        }
        fs::write(path, out).map_err(|e| format!("write failed: {e}"))?;
        Ok(())
    }

    /// Read the file back, parsing each line into a Note.
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        let contents = fs::read_to_string(path).map_err(|e| format!("read failed: {e}"))?;
        for line in contents.lines() {
            if let Some((pri, text)) = line.split_once('|') {
                let priority = Priority::parse(pri)?;
                self.add(text.to_string(), priority);
            }
        }
        Ok(())
    }
}
