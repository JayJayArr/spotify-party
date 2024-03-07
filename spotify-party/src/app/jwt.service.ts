import { Injectable } from '@angular/core';
import { jwtDecode } from 'jwt-decode';

@Injectable({
    providedIn: 'root',
})
export class JwtService {
    constructor() {}

    DecodeToken(token): { sub: string; iat: number; exp: number } {
        return jwtDecode(token);
    }
}
