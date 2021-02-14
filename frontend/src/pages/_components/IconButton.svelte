<script lang="ts">
  import { AlertCircle } from 'tabler-icons-svelte';
  export let handler;
  export let title: string;
</script>

<div class="btn" on:click={handler} title={title}>
  <slot>
      <AlertCircle/>
  </slot>
</div>

<style>
  .btn {
    text-decoration: none;  
    color: black;
    display: inline-flex;
  }

  .btn:hover {
    cursor: pointer;
  }
</style>
