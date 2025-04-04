import { Component, OnInit } from '@angular/core';
import { RouterLink, RouterOutlet } from '@angular/router';
import { MatTabsModule } from '@angular/material/tabs';
import { MatToolbarModule } from '@angular/material/toolbar';
import { SocketioService } from './socketio.service';
import { Song } from '../types';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, RouterLink, MatTabsModule, MatToolbarModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent implements OnInit {
  constructor(private socketioservice: SocketioService) { }
  title = 'spotoxide-ui';
  links = ['Current', 'Voting', 'Search'];
  activeLink = this.links[0];
  username: String = '';
  songs: Song[] = [];

  ngOnInit(): void {
    this.socketioservice.username.subscribe({
      next: (name: String) => {
        if (name) {
          this.username = name;
        }
      },
    });
  }
}
