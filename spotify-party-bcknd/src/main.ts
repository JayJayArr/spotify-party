import * as compression from 'compression';

import { AppModule } from './app.module';
import { NestFactory } from '@nestjs/core';
import helmet from 'helmet';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.enableCors();
  //app.use(helmet());
  app.use(compression());
  await app.listen(3000);
}
bootstrap();
