import { Controller, Get, Query, Redirect } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Redirect('http://localhost:4200/login')
  @Get()
  Authenticate(@Query('code') code: string): string {
    return this.appService.Authenticate(code);
  }
}
