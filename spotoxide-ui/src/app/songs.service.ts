import { EventEmitter, Injectable, Output } from '@angular/core';
import { SocketioService } from './socketio.service';
import { Song } from '../types';

@Injectable({
  providedIn: 'root',
})
export class SongsService {
  songscache: Song[] = [];
  @Output() songs = new EventEmitter();
  constructor(private socketioservice: SocketioService) {
    this.socketioservice.songs.subscribe({
      next: (songs: Song[]) => {
        if (songs) {
          this.songscache = songs;
          this.songs.emit(this.songscache);
        }
      },
    });
  }
  getSongs() {
    return this.songscache;
  }
}
