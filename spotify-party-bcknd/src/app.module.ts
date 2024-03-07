import { AuthService } from './auth/auth.service';
import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { SearchController } from './search/search.controller';
import { SearchService } from './search/search.service';
import { SongsController } from './songs/songs.controller';
import { SongsService } from './songs/songs.service';
import { VotesController } from './votes/votes.controller';
import { VotesService } from './votes/votes.service';

import { ConfigModule } from '@nestjs/config';
import { AuthController } from './auth/auth.controller';
import { AuthModule } from './auth/auth.module';
import { ScheduleModule } from '@nestjs/schedule';

@Module({
  imports: [ConfigModule.forRoot(), ScheduleModule.forRoot(), AuthModule],
  controllers: [
    AppController,
    SongsController,
    VotesController,
    SearchController,

    AuthController,
  ],
  providers: [
    AppService,
    SongsService,
    VotesService,
    SearchService,
    AuthService,
  ],
})
export class AppModule {
  authcode = '';
}
