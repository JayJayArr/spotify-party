import { Component } from '@angular/core';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { FormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { SocketioService } from '../socketio.service';

@Component({
  selector: 'app-search',
  imports: [
    MatInputModule,
    MatIconModule,
    MatButtonModule,
    FormsModule,
    MatFormFieldModule,
  ],
  templateUrl: './search.component.html',
  styleUrl: './search.component.scss',
})
export class SearchComponent {
  constructor(private socketioservice: SocketioService) { }
  searchstring = '';
  searchOnClick() {
    // event?.preventDefault();
    console.log('got search', this.searchstring);
    this.socketioservice.search(this.searchstring);
  }
}
