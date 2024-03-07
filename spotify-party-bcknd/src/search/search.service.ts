import { Injectable } from '@nestjs/common';
import QueryString = require('qs');

@Injectable()
export class SearchService {
  async search(
    searchstring: string,
  ): Promise<{ name: string; artist: string[]; uri: string }[]> {
    const SongsList: { name: string; artist: string[]; uri: string }[] = [];
    return new Promise(function (resolve, reject) {
      try {
        fetch(
          'https://api.spotify.com/v1/search?' +
            QueryString.stringify(
              {
                q: searchstring,
                type: 'track,artist',
              },
              { encode: false },
            ),
          {
            method: 'get',
            headers: {
              Authorization: 'Bearer ' + global.token,
              'Content-Type': 'application/json',
            },
          },
        )
          .then((response) => response.json())
          .then((data) => {
            if (typeof data.tracks?.items != 'undefined') {
              try {
                for (const song of data.tracks.items) {
                  const Artists: string[] = [];
                  for (let i = 0; i <= song.artists.length - 1; i++) {
                    Artists.push(song.artists[i].name);
                  }

                  SongsList.push({
                    name: song.name,
                    artist: [...Artists],
                    uri: song.uri,
                  });

                  resolve(SongsList);
                }
              } catch (error) {
                console.error(error);
              }
            }
          });
      } catch (error) {
        console.error(error);
        reject(error);
      }
    });
  }
}
