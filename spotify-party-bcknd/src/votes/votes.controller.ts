import { AuthGuard } from './../auth/auth.guard';
import { Body, Controller, Get, Post, UseGuards } from '@nestjs/common';
import { VotesService } from './votes.service';

@Controller('vote')
export class VotesController {
  constructor(private readonly appService: VotesService) {}

  //Create a matrix: RowID: SongID, Every added column is for a User and wether he has already voted for that song
  @UseGuards(AuthGuard)
  @Post()
  voteSong(@Body() body) {
    return this.appService.voteSong(
      body.name,
      body.artist,
      body.uri,
      body.username,
    );
  }
  @UseGuards(AuthGuard)
  @Get()
  getVotes() {
    return this.appService.getVotes();
  }
}
