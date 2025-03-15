import { Injectable } from '@angular/core';
import { Socket, io } from 'socket.io-client';

@Injectable({
  providedIn: 'root',
})
export class SocketioService {
  protected socket: Socket = io('ws://localhost:3000');

  constructor() {
    console.log('SocketService started');
  }
}
