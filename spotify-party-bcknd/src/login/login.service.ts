import { Injectable } from '@nestjs/common';

import QueryString = require('qs');
@Injectable()
export class LoginService {
  private redirect_uri = 'http://localhost:3000/login';
  Authenticate(code: any): any {
    if (code != '') {
      global.authcode = code;
    }

    fetch(
      'https://accounts.spotify.com/api/token?' +
        QueryString.stringify(
          {
            grant_type: 'authorization_code',
            code: global.authcode,
            redirect_uri: this.redirect_uri,
          },
          { encode: false },
        ),
      {
        method: 'post',
        headers: {
          Authorization:
            'Basic M2NmYTkwODI0MDk2NDI0ZGIwMzg4ZjJlNTY5YTI5Mzc6NTA1NGRiNGNlYTZkNDU2Njg4YTEzYjgzN2QyNDY5OTY=',
          'Content-Type': 'application/x-www-form-urlencoded',
        },
      },
    )
      .then((response) => {
        return response.json();
      })
      .then((data) => {
        global.token = data.access_token;
        const currenttime = new Date();
        global.token_expires = currenttime.setTime(
          currenttime.getTime() + data.expires_in * 1000,
        );
        global.refresh_token = data.refresh_token;
        return data;
      });
  }
}
