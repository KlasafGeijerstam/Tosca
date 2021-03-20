import { Component, OnInit } from '@angular/core';
import { studentQueue } from '../mock';

@Component({
  selector: 'app-tosca-queue',
  templateUrl: './tosca-queue.component.html',
  styleUrls: ['./tosca-queue.component.css']
})
export class ToscaQueueComponent implements OnInit {
  studentQueue = studentQueue;
  adminView = true;
  constructor() { }

  ngOnInit(): void {
  }

  signup(msg: string, url: string): void {
    this.studentQueue.push({name: 'The New Student', link: url, id: 30, message: msg, username: 'unknown'})
  }
}
