import { Component, OnInit } from '@angular/core';
import {Workspace, WorkspacesService} from '../workspaces.service';

@Component({
  selector: 'app-workspace-list',
  templateUrl: './workspace-list.component.html',
  styleUrls: ['./workspace-list.component.css']
})
export class WorkspaceListComponent implements OnInit {
  constructor(private workspacesService: WorkspacesService) { }
  workspaces: Workspace[] = [];

  ngOnInit(): void {
    this.getWorkspaces();
  }

  getWorkspaces(): void {
    this.workspacesService.getWorkspaces()
      .subscribe(workspaces => this.workspaces = workspaces);
  }

  putWorkspace(workspace: Workspace): void {
    this.workspacesService.putWorkspace(workspace);
  }

  onAddWorkspace(): void {
    this.putWorkspace({
      name: 'Workspace Name',
      creator: 'Creator',
      info: 'Workspace Info',
      workspaceId: 1,
      img: 'assets/genericCardImg.png'
    });
  }
}
