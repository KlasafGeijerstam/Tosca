import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-nav-bar',
  templateUrl: './nav-bar.component.html',
  styleUrls: ['./nav-bar.component.css']
})
export class NavBarComponent implements OnInit {
  static navigation: Array<string> = [];
  public navBar = NavBarComponent;

  constructor() { }

  ngOnInit(): void {
  }

  static push(page: string) {
    this.navigation.push(page);
  }

  static pop() {
    this.navigation.pop();
  }

}
