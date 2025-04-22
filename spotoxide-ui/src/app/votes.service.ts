import { EventEmitter, Injectable, Output } from '@angular/core';
import { SocketioService } from './socketio.service';
import { Song, User } from '../types';

@Injectable({
  providedIn: 'root',
})
export class VotesService {
  votescache: [Song, User[]][] = [
    [{ title: '', artists: [], uri: '', picture: '' }, []],
  ];
  @Output() votes = new EventEmitter<[Song, User[]][]>();

  constructor(private socketioservice: SocketioService) {
    this.socketioservice.votes.subscribe({
      next: (votes: [Song, User[]][]) => {
        if (votes) {
          if (!votes.length || this.votescache.length == 1) {
            this.votescache = [];
            // this.votescache.push(votes);
            this.votescache = [...votes];
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
