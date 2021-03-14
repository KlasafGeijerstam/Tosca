import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { WorkspaceListComponent } from './workspace-list/workspace-list.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from "@angular/material/icon";
import { MatButtonModule } from "@angular/material/button";
import { MatListModule } from "@angular/material/list";
import { ToscaQueueComponent } from './tosca-queue/tosca-queue.component';
import { ToscaQueueItemComponent } from './tosca-queue-item/tosca-queue-item.component';
import { HeaderComponent } from './header/header.component';
import { FooterComponent } from './footer/footer.component';
import { NavBarComponent } from './nav-bar/nav-bar.component';
import { ToscaSignupComponent } from './tosca-signup/tosca-signup.component';

@NgModule({
  declarations: [
    AppComponent,
    WorkspaceListComponent,
    ToscaQueueComponent,
    ToscaQueueItemComponent,
    HeaderComponent,
    FooterComponent,
    NavBarComponent,
    ToscaSignupComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MatCardModule,
    MatIconModule,
    MatButtonModule,
    MatListModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
