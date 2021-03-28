import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ToscaWorkspaceComponent } from './tosca-workspace.component';

describe('ToscaWorkspaceComponent', () => {
  let component: ToscaWorkspaceComponent;
  let fixture: ComponentFixture<ToscaWorkspaceComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ToscaWorkspaceComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ToscaWorkspaceComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
