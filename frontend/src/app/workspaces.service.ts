import { Injectable } from '@angular/core';
import {Observable, of, pipe} from 'rxjs';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import {catchError} from 'rxjs/operators';

export interface Workspace {
  workspace_id: number;
  creator: string;
  name: string;
  info: string;
  img: string;
}

@Injectable({
  providedIn: 'root'
})
export class WorkspacesService {
  private apiUrl = 'https://localhost:25674/api/workspaces';

  constructor(
    private http: HttpClient,
  ) { }

  getWorkspaces(): Observable<Workspace[]> {
    const headers = new HttpHeaders({Authorization: 'Bearer token_admin'});
    return this.http.get<Workspace[]>(this.apiUrl, {headers});/*.pipe(
      catchError(this.handleError<Workspace[]>('getWorkspaces', []))
    );*/
  }

  putWorkspace(workspace: Workspace): Observable<Workspace> {
    const headers = new HttpHeaders({Authorization: 'Bearer token_admin'});
    return this.http.post<Workspace>(this.apiUrl, workspace, {headers}); /*.pipe(
      catchError(this.handleError<Workspace>('putWorkspaces'))
    );*/
  }

  private handleError<T>(operation = 'operation', result?: T) {
    return (error: any): Observable<T> => {
      console.error(error);
      throw Error('Backend error');
    };
  }
}
