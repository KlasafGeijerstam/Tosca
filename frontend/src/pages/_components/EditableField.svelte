<script lang="ts">
  export let title: string;
  export let value: string;
  export let edit: boolean = false;
</script>

<div>
  <h4>{title}:</h4>
  {#if edit}
    <input class="test" type="text" {value}>
  {:else}
    <p>{value}</p>
  {/if}
</div>

<style>
  h4 {
    font-weight: bold;
  }

  .test {
    border-radius: 0.2em;
    border: 1px solid var(--border-light);
  }
</style>
