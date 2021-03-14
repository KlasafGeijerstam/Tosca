import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ToscaQueueItemComponent } from './tosca-queue-item.component';

describe('ToscaQueueItemComponent', () => {
  let component: ToscaQueueItemComponent;
  let fixture: ComponentFixture<ToscaQueueItemComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ToscaQueueItemComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ToscaQueueItemComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
