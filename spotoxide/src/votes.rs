use std::collections::HashMap;

use crate::{song::Song, user::User};
#[derive(Default, Clone)]
pub struct Votes(HashMap<Song, Vec<User>>);
impl Votes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
