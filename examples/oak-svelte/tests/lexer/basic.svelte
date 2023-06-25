<script>
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';

    export let name = 'Svelte';
    export let version = 3;

    let count = 0;
    const items = ['one', 'two', 'three'];
    
    // Reactive declaration
    $: doubled = count * 2;
    $: if (count > 10) {
        console.log(`Count is high: ${count}`);
    }

    function handleClick() {
        count += 1;
    }

    onMount(() => {
        console.log('Component mounted');
        return () => {
            console.log('Component destroyed');
        };
    });
</script>

<main>
    <h1>Hello {name}!</h1>
    <p>Version: {version}</p>
    
    <button on:click={handleClick}>
        Clicked {count} times
    </button>
    
    <p>
        Doubled: {doubled}
    </p>

    {#if count > 5}
        <p class="warning">Count is greater than 5</p>
    {:else}
        <p>Keep clicking...</p>
    {/if}

    <ul>
        {#each items as item, i}
            <li>{i + 1}: {item}</li>
        {/each}
    </ul>

    {#await Promise.resolve('data')}
        <p>Loading...</p>
    {:then value}
        <p>Got: {value}</p>
    {:catch error}
        <p>Error: {error.message}</p>
    {/await}
    
    <!-- Component Slot -->
    <div class="box">
        <slot>Default content</slot>
    </div>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    .warning {
        color: red;
        font-weight: bold;
    }
    
    button {
        background-color: #ff3e00;
        color: white;
        border: none;
        padding: 8px 16px;
        cursor: pointer;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>
