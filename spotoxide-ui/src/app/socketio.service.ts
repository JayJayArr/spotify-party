import { HttpClient } from '@angular/common/http';
import { EventEmitter, Injectable, OnDestroy, Output } from '@angular/core';
import { Socket, io } from 'socket.io-client';
import { Song, User } from '../types';
import { environment } from '../environments/environment';

@Injectable({
  providedIn: 'root',
})
export class SocketioService implements OnDestroy {
  token: String = '';
  usernamecache: String = '';
  songscache: Song[] = [];
  votescache: [Song, User[]][] = [
    [{ title: '', artists: [], uri: '', picture: '' }, []],
  ];

  apiurl = environment.apiBaseUrl;
  protected socket: Socket = io(`${this.apiurl}:3000`, {
    autoConnect: false,
  });
  @Output() username = new EventEmitter<string>();
  @Output() songs = new EventEmitter();
  @Output() votes = new EventEmitter();
  @Output() searchresult = new EventEmitter();

  constructor(private http: HttpClient) {
    console.log('SocketService started');
    this.init();
  }

  ngOnDestroy(): void {
    this.socket.disconnect();
  }

  async init() {
    await this.getToken();
    this.socket.on('votes', (data: [Song, User[]][]) => {
      this.votescache = data;
      this.votes.emit(data);
    });

    this.socket.on('songs', (data) => {
      this.songscache = data;
      this.songs.emit(data);
    });

    this.socket.on('search', (data) => {
      this.searchresult.emit(data);
    });

    this.socket.on('connect_error', (err) => {
      console.log(err);
    });
  }

  async search(searchstring: String) {
    this.socket.emit('search', { searchstring });
  }

  async vote(song: Song) {
    console.log('Voting for id: ', song);
    this.socket.emit('vote', song);
  }

  getUsername() {
    return this.usernamecache;
  }

  getSongs() {
    return this.songscache;
  }

  getVotes() {
    return this.votescache;
  }

  async getToken() {
    if (localStorage.getItem('token') != null) {
      let localstoragedata = localStorage.getItem('token');
      if (localstoragedata) {
        this.token = localstoragedata;
        let tokendata = await JSON.parse(
          atob(this.token.toString()?.split('.')[1]),
        );
        if (tokendata?.exp <= new Date().valueOf() / 1000) {
          this.refreshToken();
        } else {
          this.socket.auth = { token: `bearer ${this.token}` };
          this.usernamecache = tokendata?.name;
          this.username.emit(tokendata?.name);
          this.socket.connect();
          console.log('socket connected');
        }
      }
    } else {
      this.refreshToken();
    }
  }

  refreshToken() {
    this.http.post(`${this.apiurl}:3000/signin`, {}).subscribe({
      next: async (token) => {
        this.token = token.toString();
        localStorage.setItem('token', token.toString());
        let tokendata = await JSON.parse(atob(token.toString()?.split('.')[1]));
        this.socket.auth = { token: `bearer ${token}` };
        this.usernamecache = tokendata?.name;
        this.username.emit(tokendata?.name);
        this.socket.connect();
        console.log('socket connected');
      },
    });
  }
}
