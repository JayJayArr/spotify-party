import { SongsService } from './songs.service';
import { Controller, Get } from '@nestjs/common';

@Controller('songs')
export class SongsController {
  constructor(private readonly appService: SongsService) {}

  @Get()
  async getCurrent(): Promise<{ name: string; artist: string[] }[]> {
    return this.appService.getCurrent();
  }
}
