import { AppController } from './app.controller';
import { AppService } from './app.service';
import { AuthController } from './auth/auth.controller';
import { AuthModule } from './auth/auth.module';
import { AuthService } from './auth/auth.service';
import { ConfigModule } from '@nestjs/config';
import { LoginController } from './login/login.controller';
import { LoginModule } from './login/login.module';
import { LoginService } from './login/login.service';
import { Module } from '@nestjs/common';
import { ScheduleModule } from '@nestjs/schedule';
import { SearchController } from './search/search.controller';
import { SearchService } from './search/search.service';
import { SongsController } from './songs/songs.controller';
import { SongsService } from './songs/songs.service';
import { VotesController } from './votes/votes.controller';
import { VotesService } from './votes/votes.service';

@Module({
  imports: [
    ConfigModule.forRoot(),
    ScheduleModule.forRoot(),
    AuthModule,
    LoginModule,
  ],
  controllers: [
    AppController,
    SongsController,
    VotesController,
    SearchController,
    AuthController,
    LoginController,
  ],
  providers: [
    AppService,
    SongsService,
    VotesService,
    SearchService,
    AuthService,
    LoginService,
  ],
})
export class AppModule {
  authcode = '';
}
