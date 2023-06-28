<script>
  let count = 0;
</script>

<button on:click={() => count += 1}>
  Clicked {count} {count === 1 ? 'time' : 'times'}
</button>

{#snippet hello(name)}
  <p>Hello {name}!</p>
{/snippet}

{@render hello('Svelte')}
