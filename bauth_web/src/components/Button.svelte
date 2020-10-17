<script lang="typescript">
  export let label: string
</script>

<button on:click>{label}</button>

<style>
  button {
    appearance: none;
    border: none;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    min-width: 100px;
    height: 35px;
    padding-left: 15px;
    padding-right: 15px;
    font-size: 0.9rem;
    font-weight: bold;
    transition: all 0.2s;
    cursor: pointer;
    color: var(--background);
    background-color: var(--primary);
    border-radius: 3px;
    text-align: center;
  }
</style>
