import { Component, OnInit, Inject } from '@angular/core';

import { GlobalService } from './../global.service';
import { JwtService } from './../jwt.service';
import { MatSnackBar } from '@angular/material/snack-bar';
import { environment } from 'src/environments/environment';
import {
    MatDialog,
    MAT_DIALOG_DATA,
    MatDialogRef,
    MatDialogTitle,
    MatDialogContent,
    MatDialogActions,
    MatDialogClose,
} from '@angular/material/dialog';
import { MatListModule } from '@angular/material/list';
import { NgFor, NgIf } from '@angular/common';

export interface DialogData {
    Voters: string[];
}

@Component({
    selector: 'app-vote',
    templateUrl: './vote.component.html',
    styleUrls: ['./vote.component.scss'],
})
export class VoteComponent implements OnInit {
    isloaded = false;
    private token = localStorage.getItem('token');
    SongsList: {
        name: string;
        artist: string[];
        uri: string;
        votes: number;
        Voters: string[];
    }[];
    constructor(
        public globalService: GlobalService,
        private jwtService: JwtService,
        private _snackBar: MatSnackBar,
        public dialog: MatDialog
    ) {
        this.getJWT();
    }

    ngOnInit(): void {
        this.GetVotes();
    }
    GetVotes(): void {
        try {
            fetch(
                'http://' +
                    environment.BackendAddress +
                    ':' +
                    environment.BackendPort +
                    '/vote',
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
                    this.SongsList = data;
                    this.isloaded = true;
                });
        } catch (error) {
            console.log(error);
        }
    }
    Vote(name: string, artist: string[], uri: string) {
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
            this.GetVotes();
        });
    }

    openSnackBar() {
        this._snackBar.open('Thank you for your vote!', 'Party!', {
            duration: 2000,
        });
    }

    openDialog(voters: string[]): void {
        const dialogRef = this.dialog.open(VoteDialog, {
            data: { Voters: voters },
        });
        dialogRef.afterClosed().subscribe((result) => {
            console.log('The dialog was closed');
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

@Component({
    selector: 'vote-dialog',
    templateUrl: 'vote.dialog.html',
    standalone: true,
    imports: [
        MatDialogTitle,
        MatDialogContent,
        MatDialogActions,
        MatDialogClose,
        MatListModule,
        NgFor,
        NgIf,
    ],
})
export class VoteDialog {
    constructor(
        public dialogRef: MatDialogRef<VoteDialog>,
        @Inject(MAT_DIALOG_DATA) public data: DialogData
    ) {}

    onNoClick(): void {
        this.dialogRef.close();
    }
}
