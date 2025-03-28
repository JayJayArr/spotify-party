import { Component, OnInit } from '@angular/core';
import { PlaylistComponent } from '../playlist/playlist.component';
import { Song } from '../../types';
import { SocketioService } from '../socketio.service';

@Component({
  selector: 'app-currentplay',
  imports: [PlaylistComponent],
  templateUrl: './currentplay.component.html',
  styleUrl: './currentplay.component.scss',
})
export class CurrentplayComponent implements OnInit {
  constructor(private socketioservice: SocketioService) { }
  songs: Song[] = [];
  ngOnInit(): void {
    this.socketioservice.songs.subscribe({
      next: (songs: Song[]) => {
        if (songs) {
          this.songs = songs;
        }
      },
    });
  }
}
