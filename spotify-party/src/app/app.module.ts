import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';

import { LoginComponent } from './login/login.component';
import { PlaylistComponent } from './playlist/playlist.component';
import { VoteComponent } from './vote/vote.component';

import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatTabsModule } from '@angular/material/tabs';
import { MatInputModule } from '@angular/material/input';
import { MatListModule } from '@angular/material/list';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { SearchComponent } from './search/search.component';
import { MatSnackBarModule } from '@angular/material/snack-bar';
@NgModule({
    declarations: [
        AppComponent,
        LoginComponent,
        PlaylistComponent,
        VoteComponent,
        SearchComponent,
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        BrowserAnimationsModule,
        MatToolbarModule,
        MatButtonModule,
        MatTabsModule,
        MatInputModule,
        MatProgressSpinnerModule,
        MatListModule,
        MatSnackBarModule,
    ],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule {}
