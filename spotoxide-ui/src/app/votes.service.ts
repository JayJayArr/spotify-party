import { EventEmitter, Injectable, Output } from '@angular/core';
import { SocketioService } from './socketio.service';
import { Vote } from '../types';

@Injectable({
  providedIn: 'root',
})
export class VotesService {
  votescache: Vote[] = [];
  @Output() votes = new EventEmitter<Vote[]>();

  constructor(private socketioservice: SocketioService) {
    this.socketioservice.votes.subscribe({
      next: (votes: any) => {
        if (votes) {
          if (!votes.length) {
            this.votescache = [];
            this.votescache.push(votes);
          } else {
            this.votescache = [...votes];
            this.votes.emit(this.votescache);
          }
        }
      },
    });
  }

  getVotes() {
    return this.votescache;
  }
}
