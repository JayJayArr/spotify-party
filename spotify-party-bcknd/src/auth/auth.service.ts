import { Injectable } from '@nestjs/common';
import { readFileSync } from 'node:fs';
import { JwtService } from '@nestjs/jwt';
import {
  uniqueNamesGenerator,
  adjectives,
  colors,
  animals,
} from 'unique-names-generator';

@Injectable()
export class AuthService {
  constructor(private readonly jwtService: JwtService) {}
  private privateKey = readFileSync('../keys/jwtRSA256-private.pem');

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
