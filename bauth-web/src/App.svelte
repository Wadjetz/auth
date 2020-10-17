<script lang="typescript">
  import { RouterView, initPathRouter } from "@bjornlu/svelte-router"

  import Home from "./screens/Home.svelte"
  import Auth from "./screens/Auth.svelte"

  initPathRouter([
    { path: "/admin", component: Home },
    { path: "/admin/oauth/callback", component: Auth }
  ])
</script>

<RouterView />
