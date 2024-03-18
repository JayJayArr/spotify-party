import { AuthModule } from './auth/auth.module';
import { ConfigModule } from '@nestjs/config';
import { LoginModule } from './login/login.module';
import { Module } from '@nestjs/common';
import { ScheduleModule } from '@nestjs/schedule';
import { SearchModule } from './search/search.module';
import { ServeStaticModule } from '@nestjs/serve-static';
import { SongsModule } from './songs/songs.module';
import { VotesModule } from './votes/votes.module';
import { join } from 'path';

@Module({
  imports: [
    ConfigModule.forRoot(),
    ScheduleModule.forRoot(),
    AuthModule,
    LoginModule,
    AppModule,
    SongsModule,
    VotesModule,
    SearchModule,
    SearchModule,
    ServeStaticModule.forRoot({
      rootPath: join(__dirname, '../../spotify-party/dist/spotify-party'),
      renderPath: join(__dirname, '../../spotify-party/dist/spotify-party'),
      //serveRoot: join(__dirname, '../../spotify-party/dist/spotify-party'),
    }),
  ],
})
export class AppModule {
  //authcode = '';
}
