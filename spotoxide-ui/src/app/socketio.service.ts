import { HttpClient } from '@angular/common/http';
import { EventEmitter, Injectable, OnInit, Output } from '@angular/core';
import { Socket, io } from 'socket.io-client';

@Injectable({
  providedIn: 'root',
})
export class SocketioService implements OnInit {
  token: String = '';
  protected socket: Socket = io('ws://localhost:3000', {
    auth: {
      // token: `bearer ${this.token}`,
    },
  });
  @Output() username = new EventEmitter<string>();
  @Output() songs = new EventEmitter();

  constructor(private http: HttpClient) {
    console.log('SocketService started');
    this.init();
  }

  ngOnInit(): void {}

  async init() {
    this.http.post('http://localhost:3000/signin', {}).subscribe({
      next: (token) => {
        this.token = token.toString();
        this.socket.auth = { token: `bearer ${token}` };
        this.socket.connect();
        this.socket.on('username', (data) => {
          console.log('Username: ', data);
          this.username.emit(data);
        });

        this.socket.on('votes', (data) => {
          console.log('Votes: ', data);
        });

        this.socket.on('songs', (data) => {
          console.log('Songs: ', data);
        });
        console.log(this.socket);
      },
    });
  }
}
