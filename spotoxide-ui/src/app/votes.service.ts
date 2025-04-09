import { EventEmitter, Injectable, Output } from '@angular/core';
import { SocketioService } from './socketio.service';
import { Vote } from '../types';

@Injectable({
  providedIn: 'root',
})
export class VotesService {
  votescache: Vote[] = [];
  @Output() votes = new EventEmitter();

  constructor(private socketioservice: SocketioService) {
    this.socketioservice.songs.subscribe({
      next: (votes: Vote[]) => {
        if (votes) {
          this.votescache = votes;
          this.votes.emit(this.votescache);
        }
      },
    });
  }

  getSongs() {
    return this.votescache;
  }
}
