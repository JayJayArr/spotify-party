import { AuthController } from './auth.controller';
import { AuthService } from './auth.service';
import { JwtModule } from '@nestjs/jwt';
import { JwtService } from '@nestjs/jwt';
import { Module } from '@nestjs/common';
//import { jwtConstants } from './constants';
import { readFileSync } from 'node:fs';

@Module({
  imports: [
    JwtModule.register({
      global: true,
      secret: readFileSync('./src/keys/jwtRSA256-private.pem'),
      signOptions: { expiresIn: '48h' },
    }),
  ],
  controllers: [AuthController],
  providers: [AuthService, JwtService],
})
export class AuthModule {}
