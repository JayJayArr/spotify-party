use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::song::Song;
#[derive(Default, Clone, Serialize, Debug)]
pub struct SongQueue {
    last_updated: DateTime<Utc>,
    songs: VecDeque<Song>, //dont forget to truncate dis bich
}

impl SongQueue {
    pub fn new() -> Self {
        Self {
            last_updated: chrono::offset::Utc::now(),
            songs: VecDeque::new(),
        }
    }
}
