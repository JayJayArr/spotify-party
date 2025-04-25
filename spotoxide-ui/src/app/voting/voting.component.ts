import { Component } from '@angular/core';
import { Song, User } from '../../types';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { SocketioService } from '../socketio.service';

@Component({
  selector: 'app-voting',
  imports: [MatCardModule, MatButtonModule, MatIconModule],
  templateUrl: './voting.component.html',
  styleUrl: './voting.component.scss',
})
export class VotingComponent {
  votes: [Song, User[]][] = [
    [{ title: '', artists: [], uri: '', picture: '' }, []],
  ];
  username: String = '';

  constructor(private socketioservice: SocketioService) {
    this.socketioservice.votes.subscribe({
      next: (votes: [Song, User[]][]) => {
        if (votes) {
          this.votes = votes;
        }
      },
    });

    this.socketioservice.username.subscribe({
      next: (name: String) => {
        if (name) {
          this.username = name;
        }
      },
    });
  }

  ngOnInit(): void {
    this.votes = this.socketioservice.getVotes();
    this.username = this.socketioservice.getUsername();
    console.log('votes in voting component', this.votes);
  }

  voteOnClick(song: Song) {
    event?.preventDefault();
    this.socketioservice.vote(song);
  }

  checkuser = (element: User) => element.username == this.username;
}
