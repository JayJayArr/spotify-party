import { Injectable } from '@nestjs/common';
import { Cron } from '@nestjs/schedule/dist';
import QueryString = require('qs');

@Injectable()
export class VotesService {
  SongsList: {
    name: string;
    artist: string[];
    uri: string;
    votes: number;
    Voters: string[];
  }[] = [];
  voteSong(name, artist, uri, username): Promise<string> {
    return new Promise((resolve, reject) => {
      if (this.SongsList.find((Song) => Song.uri == uri) !== undefined) {
        this.SongsList.find((Song) => Song.uri == uri).votes++;
        this.SongsList.find((Song) => Song.uri == uri).Voters.push(username);

        resolve('this Song is already in the Votes');
      } else {
        try {
          this.SongsList.push({
            name: name,
            artist: [...artist],
            uri: uri,
            votes: 1,
            Voters: [username],
          });
          resolve(
            'Thanks for your suggestion, you added ' +
              name +
              'by ' +
              artist[0] +
              'to the queue',
          );
        } catch (err) {
          reject(err);
        }
      }
    });
  }
  getVotes(): { name: string; artist: string[]; uri: string; votes: number }[] {
    return this.SongsList;
  }
  @Cron('*/2 * * * *')
  pushSong() {
    let bestsong;
    if (this.SongsList[0].uri) {
      bestsong = this.SongsList.reduce((prev, current) => {
        return prev.votes > current.votes ? prev : current;
      });
    }
    const index = this.SongsList.map((item) => item.uri).indexOf(bestsong.uri);
    console.log(index);
    try {
      fetch(
        'https://api.spotify.com/v1/me/player/queue?' +
          QueryString.stringify(
            {
              uri: bestsong?.uri,
            },
            { encode: false },
          ),
        {
          method: 'post',
          headers: {
            Authorization: 'Bearer ' + global.token,
            'Content-Type': 'application/json',
          },
        },
      );
    } catch (error) {
      console.log(error);
    }
    this.SongsList.splice(index, 1);
    console.log(this.SongsList);
  }
}
