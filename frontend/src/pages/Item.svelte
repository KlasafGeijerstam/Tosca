<script lang="ts">
  import { Pencil, Trash, ClipboardList } from "tabler-icons-svelte";
  import { createEventDispatcher } from 'svelte';

  export let courseName: string;
  export let courseText: string;

  const dispatch = createEventDispatcher();

  function test() {
    dispatch('onClick', {
      text: courseName
    });
  }

</script>

<item on:click={test}>
  <info>
    <h2>{courseName}</h2>
    <p>{courseText}</p>
    <controls>
      <Pencil />
      <Trash />
    </controls>
  </info>

  <shortcut>
    <ClipboardList />
  </shortcut>
</item>

<style>
  item {
    display: flex;
    background-color: var(--dark-gray);
    margin-bottom: 1em;
    padding: 0.3em;
    box-shadow: 0.1em 0.2em 0.3em 0em darkgray;
  }

  info {
    flex-grow: 1;
  }
</style>
