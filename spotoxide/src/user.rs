use socketioxide::extract::SocketRef;
use std::fmt;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub socket: SocketRef,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}, socket: {:?}", self.username, self.socket)
    }
}
