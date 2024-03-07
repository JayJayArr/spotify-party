import { AuthService } from './auth.service';
import { Controller, Get } from '@nestjs/common';

@Controller('auth')
export class AuthController {
  constructor(private readonly jwtservice: AuthService) {}

  @Get()
  generatetoken() {
    return this.jwtservice.createJWT();
  }
}
