<script lang="typescript">
  import { oauthClient } from "../auth"
</script>

<main class="main"><a href="{oauthClient.authorizeUri()}">Login</a></main>

<style>
  .main {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }
</style>
