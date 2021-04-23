import { Component, OnInit } from '@angular/core';
import { Workspace, WorkspacesService } from '../workspaces.service';
import { MatDialog } from '@angular/material/dialog';
import { Router } from '@angular/router';
import { ToscaWorkspaceComponent } from '../tosca-workspace/tosca-workspace.component';

@Component({
  selector: 'app-workspace-list',
  templateUrl: './workspace-list.component.html',
  styleUrls: ['./workspace-list.component.css']
})
export class WorkspaceListComponent implements OnInit {
  constructor(private workspacesService: WorkspacesService, public dialog: MatDialog, private router: Router) { }
  workspaces: Workspace[] = [];
  connection = false;

  ngOnInit(): void {
    this.getWorkspaces();
    setInterval(_ => this.getWorkspaces(), 10000);
  }

  getWorkspaces(): void {
    this.workspacesService.getWorkspaces()
      .subscribe(data => {
          this.connection = true;
          this.workspaces = data; },
        error => {
          this.connection = false;
          console.log(error);
        });
  }

  putWorkspace(workspace: Workspace): void {
    this.workspacesService.putWorkspace(workspace)
      .subscribe(ws => {
        this.connection = true;
        this.workspaces.push(ws);
      },
      error => {
        this.connection = false;
        console.log(error);
      });
  }

  onAddWorkspace(): void {
    const dialogRef = this.dialog.open(AddWorkspaceDialogComponent);
    dialogRef.updateSize('50%');
    dialogRef.afterClosed().subscribe(result => {
      if (result) {
        this.putWorkspace(result);
      }
    });
  }

  onFocusWorkspace(workspace: Workspace): void {
    const dialogRef = this.dialog.open(ToscaWorkspaceComponent, { data: workspace, autoFocus: false });
    dialogRef.updateSize('75%');
  }
}

@Component({
  selector: 'app-add-workspace-dialog',
  templateUrl: './add-workspace-dialog.html',
  styleUrls: ['./add-workspace-dialog.css'],
})
export class AddWorkspaceDialogComponent {
  newWorkspace: Workspace = {
    name: '',
    creator: '',
    info: '',
    workspace_id: 0,
    img: 'assets/genericCardImg.png'
  };
}
