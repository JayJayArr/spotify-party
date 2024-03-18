import { LoginService } from './login.service';
import { Controller, Get, Query, Redirect } from '@nestjs/common';

@Controller('login')
export class LoginController {
  constructor(private readonly loginservice: LoginService) {}

  @Redirect('http://localhost:4200')
  @Get()
  Authenticate(@Query('code') code: string): string {
    return this.loginservice.Authenticate(code);
  }
}
