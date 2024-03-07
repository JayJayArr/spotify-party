import { Controller, Get } from '@nestjs/common';

import { SongsService } from './songs.service';

@Controller('songs')
export class SongsController {
  constructor(private readonly appService: SongsService) {}

  @Get()
  async getCurrent(): Promise<{ name: string; artist: string[] }[]> {
    return this.appService.getCurrent();
  }
}
