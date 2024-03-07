import { Component, OnInit } from '@angular/core';
import FetchWrapper from 'src/Fetch-Wrapper';
import { environment } from 'src/environments/environment';

@Component({
    selector: 'app-playlist',
    templateUrl: './playlist.component.html',
    styleUrls: ['./playlist.component.scss'],
})
export class PlaylistComponent implements OnInit {
    BackEnd = new FetchWrapper(
        'http://' + environment.BackendAddress + ':3000'
    );
    SongsList: { name: string; artist: string[] }[];
    isloaded: boolean = false;
    constructor() {}

    ngOnInit() {
        try {
            fetch(
                'http://' +
                    environment.BackendAddress +
                    ':' +
                    environment.BackendPort +
                    '/songs',
                {
                    method: 'Get',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                }
            )
                .then((response) => response.json())
                .then((data) => {
                    this.SongsList = data;
                    this.isloaded = true;
                });
        } catch (error) {
            console.log(error);
        }
    }
}
