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
      next: (searchresult: Song[]) => {
        if (searchresult) {
          this.searchresult = searchresult;
        }
      },
    });
  }
  searchstring = '';
  searchresult: Song[] = [];
  searchOnClick() {
    // event?.preventDefault();
    console.log('got search', this.searchstring);
    this.socketioservice.search(this.searchstring);
  }

  voteOnClick() { }
}
