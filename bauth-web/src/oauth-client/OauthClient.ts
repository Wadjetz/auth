import qs from "query-string"

export interface Storage {
  save: (key: string, value: string) => Promise<void>
  get: (key: string) => Promise<string>
  remove: (key: string) => Promise<void>
}

const defaultLocalStorage: Storage = {
  save: (key, value) => Promise.resolve(localStorage.setItem(key, value)),
  get: key => Promise.resolve(localStorage.getItem(key)),
  remove: key => Promise.resolve(localStorage.removeItem(key))
}

export interface OauthClientConfig {
  client_id: string
  server_uri: string
  redirect_uri: string
  onSuccess: (response: OauthTokenResponse) => void
  onError: (error: Error) => void
}

export interface OauthTokenResponse {
  access_token: string
  expires_in: number
  token_type: number
}

export class OauthClient {
  private storage: Storage
  private config: OauthClientConfig
  private response: Promise<OauthTokenResponse>
  constructor(config: OauthClientConfig, storage: Storage = defaultLocalStorage) {
    this.config = config
    this.storage = storage
    window.addEventListener("load", this.handleOnLoad.bind(this))
  }

  public authorizeUri(scope?: string, state?: string): string {
    return `${this.config.server_uri}/authorize?client_id=${this.config.client_id}&response_type=code&redirect_uri=${this.config.redirect_uri}`
  }

  public login(scope?: string, state?: string) {
    window.location.assign(this.authorizeUri(scope, state))
  }

  private handleOnLoad(event: Event) {
    const url = (event.target as any).URL as string
    if (url.startsWith(this.config.redirect_uri)) {
      const code = qs.parseUrl(url).query.code
      console.log("code detected", code)
      if (code) {
        fetch(
          `${this.config.server_uri}/token?client_id=${this.config.client_id}&grant_type=authorization_code&redirect_uri=${this.config.redirect_uri}&code=${code}`,
          {
            method: "POST"
          }
        )
          .then(response => response.json())
          .then(response => {
            console.log("code detected response", code)
            this.storage
              .save("OAUTH_ACCESS_TOKEN", JSON.stringify(response))
              .then(() => this.config.onSuccess(response))
          })
          .catch(this.config.onError)
      }
    }
  }
}
