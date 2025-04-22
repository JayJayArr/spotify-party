import { Component } from '@angular/core';
import { Song, Vote } from '../../types';
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
  votes: any[] = [];
  constructor(
    private votesService: VotesService,
    private socketioservice: SocketioService,
  ) {
    this.votesService.votes.subscribe({
      next: (votes: Vote[]) => {
        console.log('votes in component from subscription', votes);
        if (votes) {
          this.votes = votes;
        }
      },
    });
  }
  ngOnInit(): void {
    this.votes = this.votesService.getVotes();
    console.log('votes in component', this.votes);
  }

  voteOnClick(song: Song) {
    this.socketioservice.vote(song);
  }
}
