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
          console.log('username in voting component:', this.username);
        }
      },
    });
  }

  ngOnInit(): void {
    this.votes = this.socketioservice.getVotes();
    this.username = this.socketioservice.getUsername();
    console.log('votes in voting component', this.votes);
    console.log('username in voting component', this.username);
    // TODO: Compare the object here, objects are compared by reference, not by value
  }

  voteOnClick(song: Song) {
    this.socketioservice.vote(song);
  }

  checkUsername(user: User) {
    return user.username === this.username;
  }

  checkuser = (element: User) => element.username == this.username;
}
