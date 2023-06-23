<script>
  let name = 'world';
</script>

<h1>Hello {name}!</h1>

{#if name === 'world'}
  <p>Welcome to Svelte.</p>
{:else}
  <p>Hello {name}.</p>
{/if}

<button on:click={() => name = 'svelte'}>
  Change name
</button>

<style>
  h1 {
    color: purple;
  }
</style>
