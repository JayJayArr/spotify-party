export interface Song {
  title: String;
  artists: String[];
  picture: String;
  uri: String;
}

// export type Vote = [
//   data: {
//     title: String;
//     artists: String[];
//     picture: String;
//     uri: String;
//   },
//   users: String[],
// ];
export interface User {
  username: String;
}
