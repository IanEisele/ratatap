use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{error::Error, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct TestResult {
    pub wpm: f64,
    pub accuracy: f64,
    pub timestamp: DateTime<Utc>,
    pub duration_secs: u64,
    #[serde(default)]
    pub char_errors: HashMap<char, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ProgressData {
    pub results: Vec<TestResult>,
}

impl ProgressData {
    pub fn load() -> Self {
        let path = Self::get_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(progress) = serde_json::from_str(&data) {
                    return progress;
                }
            }
        }
        ProgressData {
            results: Vec::new(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".ratatap");
        path.push("progress.json");
        path
    }

    pub fn clear_history() -> Result<(), Box<dyn Error>> {
        let path = Self::get_path();
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn average_wpm(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            self.results.iter().map(|r| r.wpm).sum::<f64>() / self.results.len() as f64
        }
    }

    pub fn average_accuracy(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            self.results.iter().map(|r| r.accuracy).sum::<f64>() / self.results.len() as f64
        }
    }

    pub fn get_wpm_history(&self, count: usize) -> Vec<u64> {
        self.results
            .iter()
            .rev()
            .take(count)
            .map(|r| r.wpm as u64)
            .rev()
            .collect()
    }

    pub fn get_char_error_analysis(&self) -> HashMap<char, CharStats> {
        let mut char_stats: HashMap<char, CharStats> = HashMap::new();

        for result in &self.results {
            for (ch, &error_count) in &result.char_errors {
                let stats = char_stats.entry(*ch).or_insert(CharStats {
                    total_errors: 0,
                    total_appearances: 0,
                });
                stats.total_errors += error_count;
                stats.total_appearances += 1;
            }
        }

        char_stats
    }


    pub fn get_weakest_chars(&self, count: usize) -> Vec<(char, f64)> {
        let char_analysis = self.get_char_error_analysis();
        let mut chars: Vec<(char, f64)> = char_analysis
            .iter()
            .filter(|(_, stats)| stats.total_appearances > 0)
            .map(|(ch, stats)| {
                let error_rate =
                    stats.total_errors as f64 / stats.total_appearances.max(1) as f64;
                (*ch, error_rate)
            })
            .collect();

        chars.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        chars.into_iter().take(count).collect()
    }
}

#[derive(Debug, Clone)]
pub struct CharStats {
    pub total_errors: usize,
    pub total_appearances: usize,
}

