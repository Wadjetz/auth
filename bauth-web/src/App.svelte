<script lang="typescript">
  import { RouterView, initHashRouter } from "@bjornlu/svelte-router"

  import Home from "./screens/Home.svelte"

  initHashRouter([
    { path: "/", component: Home },
  ])
</script>

<RouterView />
