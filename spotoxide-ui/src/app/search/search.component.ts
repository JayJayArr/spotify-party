import { Component } from '@angular/core';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { FormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { SocketioService } from '../socketio.service';
import { Song } from '../../types';
import { MatCardModule } from '@angular/material/card';

@Component({
  selector: 'app-search',
  imports: [
    MatInputModule,
    MatIconModule,
    MatButtonModule,
    FormsModule,
    MatFormFieldModule,
    MatCardModule,
  ],
  templateUrl: './search.component.html',
  styleUrl: './search.component.scss',
})
export class SearchComponent {
  constructor(private socketioservice: SocketioService) {
    this.socketioservice.searchresult.subscribe({
      next: (searchresult: any) => {
        if (searchresult) {
          let collection: any = [];
          searchresult.items.forEach((song: any) => {
            let artists: String[] = [];
            song.artists.forEach((artist: any) => {
              artists.push(artist.name);
            });
            let newsong: Song = {
              title: song.name,
              artists: artists,
              uri: song.uri,
              picture: song.album.images[0].url,
            };
            collection.push(newsong);
          });
          this.searchresult = collection;
        }
      },
    });
  }
  searchstring = '';
  searchresult: Song[] = [];
  searchOnClick() {
    this.socketioservice.search(this.searchstring);
  }

  voteOnClick(song: Song) {
    this.socketioservice.vote(song);
  }
}
