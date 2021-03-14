import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ToscaQueueComponent } from './tosca-queue.component';

describe('ToscaQueueComponent', () => {
  let component: ToscaQueueComponent;
  let fixture: ComponentFixture<ToscaQueueComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ToscaQueueComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ToscaQueueComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
