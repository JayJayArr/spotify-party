import { Routes } from '@angular/router';
import { CurrentplayComponent } from './currentplay/currentplay.component';
import { VotingComponent } from './voting/voting.component';
import { SearchComponent } from './search/search.component';

export const routes: Routes = [
  { path: 'Current', component: CurrentplayComponent },
  { path: 'Voting', component: VotingComponent },
  { path: 'Search', component: SearchComponent },
  { path: '**', redirectTo: 'Current' },
];
