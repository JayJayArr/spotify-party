import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MatTabsModule } from '@angular/material/tabs';
import { MatToolbarModule } from '@angular/material/toolbar';
import { SocketioService } from './socketio.service';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, MatTabsModule, MatToolbarModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  constructor(private socketioservice: SocketioService) {}
  title = 'spotoxide-ui';
}
