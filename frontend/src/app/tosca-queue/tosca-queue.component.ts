import { Component, OnInit } from '@angular/core';
import { studentQueue } from '../studentqueue'

@Component({
  selector: 'app-tosca-queue',
  templateUrl: './tosca-queue.component.html',
  styleUrls: ['./tosca-queue.component.css']
})
export class ToscaQueueComponent implements OnInit {
  studentQueue = studentQueue
  constructor() { }

  ngOnInit(): void {
  }

}
