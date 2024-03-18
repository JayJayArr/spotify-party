import { Controller, Get } from '@nestjs/common';

import { AuthService } from './auth.service';
@Controller('auth')
export class AuthController {
  constructor(private readonly jwtservice: AuthService) {}

  @Get()
  generatetoken() {
    return this.jwtservice.createJWT();
  }
}
