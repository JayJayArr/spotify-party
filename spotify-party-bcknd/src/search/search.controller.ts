import { AuthGuard } from './../auth/auth.guard';
import { SearchService } from './search.service';
import { Controller, Get, Query, UseGuards } from '@nestjs/common';

@Controller('search')
export class SearchController {
  constructor(private readonly appService: SearchService) {}

  @UseGuards(AuthGuard)
  @Get()
  async search(@Query('q') requeststring: string) {
    return this.appService.search(requeststring);
  }
}
