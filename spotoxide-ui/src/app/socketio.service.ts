import { HttpClient } from '@angular/common/http';
import { EventEmitter, Injectable, OnDestroy, Output } from '@angular/core';
import { Socket, io } from 'socket.io-client';

@Injectable({
  providedIn: 'root',
})
export class SocketioService implements OnDestroy {
  token: String = '';
  protected socket: Socket = io('ws://localhost:3000', {
    autoConnect: false,
  });
  @Output() username = new EventEmitter<string>();
  @Output() songs = new EventEmitter();

  constructor(private http: HttpClient) {
    console.log('SocketService started');
    this.init();
  }

  ngOnDestroy(): void {
    this.socket.disconnect();
  }

  async init() {
    this.http.post('http://localhost:3000/signin', {}).subscribe({
      next: async (token) => {
        this.token = token.toString();
        let tokendata = await JSON.parse(atob(token.toString()?.split('.')[1]));
        this.socket.auth = { token: `bearer ${token}` };
        this.username.emit(tokendata?.name);

        this.socket.on('votes', (data) => {
          console.log('Votes: ', data);
        });

        this.socket.on('songs', (data) => {
          this.songs.emit(data);
          console.log('Songs: ', data);
        });
        this.socket.on('connect_error', (err) => {
          console.log(err);
        });
      },
    });
  }
}
