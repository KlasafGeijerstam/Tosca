import { Component, OnInit } from '@angular/core';
import { workspaces } from "../workspaces";

@Component({
  selector: 'app-workspace-list',
  templateUrl: './workspace-list.component.html',
  styleUrls: ['./workspace-list.component.css']
})
export class WorkspaceListComponent implements OnInit {
  workspaces = workspaces;
  constructor() { }


  ngOnInit(): void {
  }

}
