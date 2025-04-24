use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use serde::Serialize;
use spotify_rs::model::{PlayableItem, player::Queue};

use crate::song::Song;
#[derive(Default, Clone, Serialize, Debug)]
pub struct SongQueue {
    //last_updated will be checked for the broadcast,
    //if the queue was not updated in the relevant timeframe, why broadcast it
    last_updated: DateTime<Utc>,
    songs: VecDeque<Song>,
}

impl SongQueue {
    pub fn new() -> Self {
        Self {
            last_updated: chrono::Utc::now(),
            songs: VecDeque::new(),
        }
    }
    //used to update the songs without replacing them
    pub fn push(&mut self, song: Song) {
        self.last_updated = chrono::Utc::now();
        self.songs.push_back(song);
        //truncate the songs to be a max of
        self.songs.truncate(20);
    }

    // pub fn replace(&mut self, songs: Vec<Song>) {
    //     self.last_updated = chrono::offset::Utc::now();
    //     self.songs.clear();
    //     self.songs.append(&mut VecDeque::from(songs));
    // }
    //
    // pub fn skip(&mut self) {
    //     self.last_updated = chrono::offset::Utc::now();
    //     self.songs.pop_front();
    // }

    pub fn get(&self) -> VecDeque<Song> {
        self.songs.clone()
    }
}

impl From<Queue> for SongQueue {
    fn from(value: Queue) -> Self {
        let mut queue = SongQueue::new();
        if let Some(song) = value.currently_playing {
            match song {
                PlayableItem::Track(track) => {
                    queue.push(track.into());
                }
                PlayableItem::Episode(_) => {}
            }
        };
        for song in value.queue {
            match song {
                PlayableItem::Track(track) => {
                    queue.push(track.into());
                }
                PlayableItem::Episode(_) => {}
            }
        }
        queue
    }
}
