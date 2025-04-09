export interface Song {
  title: String;
  artists: String[];
  picture: String;
  uri: String;
}

export interface Vote {
  songid: String;
  users: String[];
}
