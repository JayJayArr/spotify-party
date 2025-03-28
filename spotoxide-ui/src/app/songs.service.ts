import { Injectable, OnInit } from '@angular/core';
import { SocketioService } from './socketio.service';
import { Song } from '../types';

@Injectable({
  providedIn: 'root',
})
export class SongsService implements OnInit {
  songs: Song[] = [];
  constructor(private socketioservice: SocketioService) {}
  ngOnInit(): void {
    this.socketioservice.songs.subscribe({
      next: (songs: Song[]) => {
        if (songs) {
          console.log(songs);
          this.songs = songs;
        }
      },
    });
  }
}
