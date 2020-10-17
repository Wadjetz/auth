import { OauthClient } from "./oauth-client/OauthClient"
import { navigate } from "@bjornlu/svelte-router"

export const oauthClient = new OauthClient({
  client_id: (window as any).BAUTH_CLIENT_ID,
  redirect_uri: "http://localhost:4100/admin/oauth/callback",
  server_uri: window.location.origin,
  onSuccess: response => {
    console.log("onSuccess", response)
    navigate("/admin")
  },
  onError: error => {
    console.log("onError", error)
  }
})
