use std::collections::VecDeque;

use chrono::{DateTime, Utc};

use crate::song::Song;
#[derive(Default)]
pub struct SongQueue {
    last_updated: DateTime<Utc>,
    songs: VecDeque<Song>, //dont forget to truncate dis bich
}
