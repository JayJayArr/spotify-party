import { Component } from '@angular/core';
import { Song, User } from '../../types';
import { VotesService } from '../votes.service';
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
  constructor(
    private votesService: VotesService,
    private socketioservice: SocketioService,
  ) {
    this.votesService.votes.subscribe({
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
    this.votes = this.votesService.getVotes();
    this.username = this.socketioservice.getUsername();
    // console.log('votes in component', this.votes);
    // TODO: Compare the object here, objects are compared by reference, not by value
    this.votes.forEach((vote) => {
      console.log(vote[1][0]);
      console.log({ username: this.username });
      if (vote[1].includes({ username: this.username })) {
        console.log(vote[1], 'includes username');
      }
    });
    // console.log('username in component', this.username);
  }

  voteOnClick(song: Song) {
    this.socketioservice.vote(song);
  }
}
