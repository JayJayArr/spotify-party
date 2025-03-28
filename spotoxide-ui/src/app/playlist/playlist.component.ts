import { Component, Input } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { Song } from '../../types';

@Component({
  selector: 'app-playlist',
  imports: [MatCardModule],
  templateUrl: './playlist.component.html',
  styleUrl: './playlist.component.scss',
})
export class PlaylistComponent {
  @Input() songs: Song[] = [];
}
