import { Component, Input, ChangeDetectionStrategy } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { Song } from '../../types';

@Component({
  selector: 'app-playlist',
  imports: [MatCardModule],
  templateUrl: './playlist.component.html',
  changeDetection: ChangeDetectionStrategy.Eager,
  styleUrl: './playlist.component.scss',
})
export class PlaylistComponent {
  @Input() songs: Song[] = [];
}
