import { Routes } from '@angular/router';
import { CurrentplayComponent } from './currentplay/currentplay.component';

export const routes: Routes = [
  { path: 'Current', component: CurrentplayComponent },
  { path: 'Voting', component: CurrentplayComponent },
  { path: 'Search', component: CurrentplayComponent },
  { path: '**', redirectTo: 'Current' },
];
