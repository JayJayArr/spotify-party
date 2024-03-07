import { GlobalService } from './global.service';
import { Component } from '@angular/core';
import { environment } from '../environments/environment';

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
})
export class AppComponent {
    constructor(public globalService: GlobalService) {}

    ngOnInit(): void {
        let username = this.globalService.username;
    }
    title = 'spotify-party';
    links = ['Playlist', 'Voting', 'Search'];
    activeLink = this.links[0];
}
