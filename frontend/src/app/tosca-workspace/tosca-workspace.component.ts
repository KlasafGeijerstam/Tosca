import { Component, Inject, OnInit } from '@angular/core';
import { Workspace } from '../workspaces.service';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-tosca-workspace',
  templateUrl: './tosca-workspace.component.html',
  styleUrls: ['./tosca-workspace.component.css']
})
export class ToscaWorkspaceComponent implements OnInit {
  workspace: Workspace;
  constructor(@Inject(MAT_DIALOG_DATA) public data: Workspace) {
    this.workspace = data;
  }

  ngOnInit(): void { }
}
