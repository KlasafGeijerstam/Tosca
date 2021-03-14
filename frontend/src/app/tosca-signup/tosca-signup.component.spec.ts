import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ToscaSignupComponent } from './tosca-signup.component';

describe('ToscaSignupComponent', () => {
  let component: ToscaSignupComponent;
  let fixture: ComponentFixture<ToscaSignupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ToscaSignupComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ToscaSignupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
