import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CurrentplayComponent } from './currentplay.component';

describe('CurrentplayComponent', () => {
  let component: CurrentplayComponent;
  let fixture: ComponentFixture<CurrentplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CurrentplayComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CurrentplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
