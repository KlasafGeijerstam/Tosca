import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { WorkspaceListComponent } from './workspace-list/workspace-list.component';
import { ToscaQueueComponent } from './tosca-queue/tosca-queue.component';
import { ToscaSignupComponent } from './tosca-signup/tosca-signup.component';

const routes: Routes = [
  {
    path: '',
    component: WorkspaceListComponent,
  },
  {
    path: 'queues',
    component: ToscaQueueComponent,
  },
  {
    path: 'signups',
    component: ToscaSignupComponent,
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
