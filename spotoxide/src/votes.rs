use std::collections::HashMap;

use crate::{song::Song, user::User};
#[derive(Default)]
struct Votes(HashMap<Song, Vec<User>>);
