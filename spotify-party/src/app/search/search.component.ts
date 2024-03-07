import { environment } from 'src/environments/environment';
import { GlobalService } from './../global.service';
import { JwtService } from './../jwt.service';
import { Component, OnInit } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import QueryString = require('qs');

@Component({
    selector: 'app-search',
    templateUrl: './search.component.html',
    styleUrls: ['./search.component.scss'],
})
export class SearchComponent implements OnInit {
    SongsList: { name: string; artist: string[]; uri: string }[] = [];
    private token = localStorage.getItem('token');
    isloaded = false;
    hasSearched = false;
    constructor(
        private globalService: GlobalService,
        private _snackBar: MatSnackBar,
        private jwtService: JwtService
    ) {
        this.getJWT();
    }
    ngOnInit(): void {}

    Search(searchstring: string) {
        fetch(
            'http://' +
                environment.BackendAddress +
                ':' +
                environment.BackendPort +
                '/search?' +
                QueryString.stringify(
                    {
                        q: searchstring,
                    },
                    { encode: false }
                ),
            {
                method: 'Get',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: 'Bearer ' + this.token,
                },
            }
        )
            .then((response) => response.json())
            .then((data) => {
                this.hasSearched = true;
                this.SongsList = data;
                this.isloaded = true;
            });
    }

    Suggest(name: string, artist: string[], uri: string) {
        fetch(
            'http://' +
                environment.BackendAddress +
                ':' +
                environment.BackendPort +
                '/vote',
            {
                method: 'Post',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: 'Bearer ' + this.token,
                },
                body: JSON.stringify({
                    name: name,
                    artist: artist,
                    uri: uri,
                    username: this.globalService.username,
                }),
            }
        ).then((response) => {
            this.SongsList = [];
            this.isloaded = false;
            this.hasSearched = false;
        });
    }

    openSnackBar() {
        this._snackBar.open('Thank you for your Suggestion!', 'Party!', {
            duration: 2000,
        });
    }

    getJWT() {
        if (!localStorage.getItem('token')) {
            try {
                fetch(
                    'http://' +
                        environment.BackendAddress +
                        ':' +
                        environment.BackendPort +
                        '/auth',
                    {
                        method: 'Get',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                    }
                )
                    .then((response) => response.json())
                    .then((data) => {
                        localStorage.setItem('token', data.access_token);
                        this.globalService.username =
                            this.jwtService.DecodeToken(data.access_token)?.sub;
                    });
            } catch (error) {
                console.log(error);
            }
        } else {
            let token = localStorage.getItem('token');
            this.globalService.username =
                this.jwtService.DecodeToken(token)?.sub;
        }
    }
}
