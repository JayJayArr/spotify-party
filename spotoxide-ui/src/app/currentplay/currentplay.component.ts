import { Component, OnInit } from '@angular/core';
import { PlaylistComponent } from '../playlist/playlist.component';
import { Song } from '../../types';
import { MatIconModule } from '@angular/material/icon';
import { SocketioService } from '../socketio.service';

@Component({
  selector: 'app-currentplay',
  imports: [PlaylistComponent, MatIconModule],
  templateUrl: './currentplay.component.html',
  styleUrl: './currentplay.component.scss',
})
export class CurrentplayComponent implements OnInit {
  songs: Song[] = [];
  constructor(private socketioservice: SocketioService) {
    this.socketioservice.songs.subscribe({
      next: (songs: Song[]) => {
        if (songs) {
          this.songs = songs;
        }
      },
    });
  }
  ngOnInit(): void {
    this.songs = this.socketioservice.getSongs();
  }
}
