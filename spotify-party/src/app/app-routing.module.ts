import { SearchComponent } from './search/search.component';
import { VoteComponent } from './vote/vote.component';
import { PlaylistComponent } from './playlist/playlist.component';
import { AppComponent } from './app.component';
import { LoginComponent } from './login/login.component';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

const routes: Routes = [
    { path: '', component: PlaylistComponent },
    { path: 'login', component: LoginComponent },
    { path: 'voting', component: VoteComponent },
    { path: 'search', component: SearchComponent },
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule],
})
export class AppRoutingModule {}
