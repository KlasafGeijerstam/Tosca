<script lang="ts">
  import ItemList from "./ItemList.svelte";
  export let title: string;
</script>

<container>
  <container-title>
    <h2>{title}</h2>
  </container-title>
  <slot>
    <p>There is nothing to show :(</p>
  </slot>
</container>

<style>
  container {
    margin-top: 2em;
    margin-left: 2em;
    margin-right: 2em;
    display: block;
    box-shadow: 0em 0.3em 0.5em 0.1em darkgray;
  }

  container-title {
    color: white;
    background-color: var(--background-dark);
    display: flex;
    align-items: center;
    height: 3em;
  }

  container-title > * {
    margin-left: 1em;
  }
</style>
