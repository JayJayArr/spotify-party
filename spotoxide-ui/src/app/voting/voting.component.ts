import { Component } from '@angular/core';
import { Vote } from '../../types';
import { VotesService } from '../votes.service';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-voting',
  imports: [MatCardModule, MatButtonModule, MatIconModule],
  templateUrl: './voting.component.html',
  styleUrl: './voting.component.scss',
})
export class VotingComponent {
  votes: Vote[] = [];
  constructor(private votesService: VotesService) {
    this.votesService.votes.subscribe({
      next: (votes: Vote[]) => {
        if (votes) {
          this.votes = votes;
        }
      },
    });
  }
  ngOnInit(): void {
    this.votes = this.votesService.getVotes();
  }

  voteOnClick() { }
}
