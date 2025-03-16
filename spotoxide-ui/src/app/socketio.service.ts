import { Injectable, OnInit } from '@angular/core';
import { Socket, io } from 'socket.io-client';

@Injectable({
  providedIn: 'root',
})
export class SocketioService implements OnInit {
  protected socket: Socket = io('ws://localhost:3000');

  constructor() {
    console.log('SocketService started');
    this.init();
  }

  ngOnInit(): void {}

  init() {
    this.socket.connect();
    this.socket.on('username', (data) => {
      console.log('Username: ', data);
    });

    this.socket.on('votes', (data) => {
      console.log('Votes: ', data);
    });

    this.socket.on('songs', (data) => {
      console.log('Songs: ', data);
    });
    console.log(this.socket);
    this.getUsername();
  }
  getUsername() {
    console.log('requesting username');
    this.socket.emit('request-username');
  }
}
