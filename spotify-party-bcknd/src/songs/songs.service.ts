import { Injectable } from '@nestjs/common';

@Injectable()
export class SongsService {
  async getCurrent(): Promise<{ name: string; artist: string[] }[]> {
    return new Promise(function (resolve, reject) {
      const SongsList: { name: string; artist: string[] }[] = [];
      try {
        fetch('https://api.spotify.com/v1/me/player/queue', {
          method: 'get',
          headers: {
            Authorization: 'Bearer ' + global.token,
            'Content-Type': 'application/json',
          },
        })
          .then((response) => response.json())
          .then((data) => {
            if (typeof data?.currently_playing?.name != 'undefined') {
              SongsList.push({
                name: data.currently_playing.name,
                artist: [data.currently_playing.artists[0].name],
              });
              for (
                let i = 1;
                i <= data.currently_playing.artists.length - 1;
                i++
              ) {
                SongsList[0].artist.push(
                  data.currently_playing.artists[i].name,
                );
              }
              if (typeof data.queue[0].name != 'undefined') {
                try {
                  for (const song of data.queue) {
                    const Artists: string[] = [];
                    for (let i = 0; i <= song.artists.length - 1; i++) {
                      Artists.push(song.artists[i].name);
                    }

                    SongsList.push({
                      name: song.name,
                      artist: [...Artists],
                    });
                  }
                } catch (error) {
                  console.log(error);
                }
              }
            } else {
              SongsList.push({
                name: 'No song is played at the moment',
                artist: [''],
              });
            }

            resolve(SongsList);
            //TODO: return the correct value here
          });
      } catch {
        reject('SongsService failed');
      }
    });
  }
}
