import { Component, OnInit, Input } from '@angular/core';

@Component({
  selector: 'app-tosca-queue-item',
  templateUrl: './tosca-queue-item.component.html',
  styleUrls: ['./tosca-queue-item.component.css']
})
export class ToscaQueueItemComponent implements OnInit {
  @Input() id: number;
  @Input() name: string;
  @Input() username: string;
  @Input() message: string;
  @Input() link: string;

  constructor() { }

  ngOnInit(): void {
  }

}
