export default class FetchWrapper {
  baseURL: string;
  constructor(baseURL: string) {
    this.baseURL = baseURL;
  }

  get(endpoint: string): Promise<any> {
    return fetch(this.baseURL + endpoint).then((response) => response.json());
  }

  put(endpoint: string, body) {
    return this._send('put', endpoint, body);
  }

  post(endpoint: string, body) {
    return this._send('post', endpoint, body);
  }

  delete(endpoint: string, body) {
    return this._send('delete', endpoint, body);
  }

  _send(method: string, endpoint: string, body) {
    return fetch(this.baseURL + endpoint, {
      method,
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    }).then((response) => response.json());
  }
}
