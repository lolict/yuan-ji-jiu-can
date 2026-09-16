//! rhyme.rs — 韵母工具桩

pub struct RhymeUtils;

impl RhymeUtils {
    pub fn new() -> Self { Self }
}

pub static RHYME_TABLE: &[&str] = &[
    "a", "o", "e", "i", "u", "v",
    "ai", "ei", "ui", "ao", "ou", "iu",
    "ie", "ve", "er", "an", "en", "in", "un", "vn",
    "ang", "eng", "ing", "ong",
];
