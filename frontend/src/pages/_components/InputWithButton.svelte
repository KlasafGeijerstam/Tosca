<script lang="ts">
  import { Plus } from 'tabler-icons-svelte'; 
  
  export let placeholder: string;
  export let handler;
  export let title: string;
  let value: string;

  function check_enter(event: KeyboardEvent) {
    if (event.key == 'Enter') {
      handler(value);
    }
  }
</script>
<div class="addmod">
  <input type="text" placeholder={placeholder} bind:value={value} on:keydown={check_enter}>
  <button on:click="{() => handler(value)}" title={title}>
  <slot>
    <Plus color="white"/>
  </slot>
  </button>
</div>
<style>
  .addmod {
    display: flex;
    height: 2em;
  }

  .addmod > input {
    border-radius: 0.2em 0em 0em 0.2em;
    border: 1px solid var(--border-light);
    border-right: none;
  }

  .addmod > button {
    border-radius: 0em 0.2em 0.2em 0em;
    border: 1px solid var(--border-light);
    background-color: var(--success);
    display: flex;
    align-items: center;
    cursor: pointer;
  }
</style>
