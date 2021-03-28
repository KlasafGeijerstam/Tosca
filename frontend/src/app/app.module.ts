import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { AddWorkspaceDialogComponent, WorkspaceListComponent } from './workspace-list/workspace-list.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatSlideToggleModule } from '@angular/material/slide-toggle';
import { MatListModule } from '@angular/material/list';
import { FormsModule } from '@angular/forms';
import { ToscaQueueComponent } from './tosca-queue/tosca-queue.component';
import { ToscaQueueItemComponent } from './tosca-queue-item/tosca-queue-item.component';
import { HeaderComponent } from './header/header.component';
import { FooterComponent } from './footer/footer.component';
import { NavBarComponent } from './nav-bar/nav-bar.component';
import { ToscaSignupComponent } from './tosca-signup/tosca-signup.component';
import { MatDividerModule } from '@angular/material/divider';
import { MatDialogModule } from '@angular/material/dialog';
import { ToscaWorkspaceComponent } from './tosca-workspace/tosca-workspace.component';

@NgModule({
  declarations: [
    AppComponent,
    WorkspaceListComponent,
    AddWorkspaceDialogComponent,
    ToscaQueueComponent,
    ToscaQueueItemComponent,
    HeaderComponent,
    FooterComponent,
    NavBarComponent,
    ToscaSignupComponent,
    ToscaWorkspaceComponent,
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MatCardModule,
    MatIconModule,
    MatButtonModule,
    MatInputModule,
    MatSlideToggleModule,
    MatListModule,
    FormsModule,
    MatDividerModule,
    MatDialogModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
