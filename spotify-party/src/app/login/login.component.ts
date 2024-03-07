import { Component, OnInit } from '@angular/core';

import FetchWrapper from 'src/Fetch-Wrapper';
import QueryString = require('qs');
import { compileNgModule } from '@angular/compiler';

@Component({
    selector: 'app-login',
    templateUrl: './login.component.html',
    styleUrls: ['./login.component.scss'],
})
export class LoginComponent implements OnInit {
    href: string = '';
    loggedin: boolean;
    scopes: string =
        'user-read-currently-playing user-read-playback-state user-modify-playback-state';
    SpotifyAuth = new FetchWrapper('https://accounts.spotify.com');
    SpotifyAPI = new FetchWrapper('https://api.spotify.com');
    BackEnd = new FetchWrapper('https://localhost:3000');

    client_id = '3cfa90824096424db0388f2e569a2937';
    redirect_uri = 'http://localhost:3000';

    constructor() {}

    ngOnInit() {}
    LoginUser(username: string, password: string): void {
        if (username == 'Admin' && password == 'Admin123') {
            this.loggedin = true;
        }
    }

    authorize = async () => {
        window.location.href =
            'https://accounts.spotify.com/authorize?' +
            QueryString.stringify({
                response_type: 'code',
                client_id: this.client_id,
                redirect_uri: this.redirect_uri,
                scope: this.scopes,
                //TODO: Add scopes variable for searching songs and accessing player
            });
    };
    /**


    login = async () => {
        let response = await this.SpotifyAPI.get(
            '/login?' +
                QueryString.stringify({
                    response_type: 'code',
                    client_id: this.client_id,
                    redirect_uri: this.redirect_uri,
                })
        );
        console.log(response);
        return response;
    };
*/
}
