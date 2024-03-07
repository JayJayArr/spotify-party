import {
  adjectives,
  animals,
  colors,
  uniqueNamesGenerator,
} from 'unique-names-generator';

import { Injectable } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { readFileSync } from 'node:fs';

@Injectable()
export class AuthService {
  constructor(private readonly jwtService: JwtService) {}
  private privateKey = readFileSync('./src/keys/jwtRSA256-private.pem');

  async createJWT() {
    const payload = {
      sub: uniqueNamesGenerator({
        dictionaries: [adjectives, colors, animals],
      }), // big_red_donkey
    };
    console.log(payload);
    return {
      access_token: await this.jwtService.signAsync(payload),
    };
    //jwt.sign(payload, process.env.SECRET_KEY);
  }
}
