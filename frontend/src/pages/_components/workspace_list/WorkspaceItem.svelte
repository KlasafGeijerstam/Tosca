<script lang="ts">
  import { Pencil, Trash, ClipboardList } from "tabler-icons-svelte";
  import { createEventDispatcher } from 'svelte';
  import IconLink from '../IconLink.svelte';
  import { url } from '@roxi/routify';

  export let workspace_id: string;
  export let name: string;
  export let info: string;
  export let active_list: string?;
  const dispatch = createEventDispatcher();

  function test() {
    dispatch('onClick', {
      text: name
    });
  }

</script>

<a href="{$url() + '/' + workspace_id}" class="item" on:click={test}>
  <info>
    <h2>{name}</h2>
    <p>{info}</p>
    <!--controls>
      <IconLink link="https://google.com", title="I like to eat food">
        <Pencil size="2em"/>
      </IconLink>
      <IconLink link="https://google.com", title="I like to eat food">
        <Trash size="2em"/>
      </IconLink>
    </controls-->
  </info>
  {#if active_list}
    <shortcut>
      <IconLink link="{active_list}" title="Go to help list">
        <ClipboardList size="2em"/>
      </IconLink>
    </shortcut>
  {/if}
</a>

<style>
  .item {
    display: flex;
    background-color: var(--dark-gray);
    margin-bottom: 1em;
    padding: 0.3em;
    box-shadow: 0.1em 0.2em 0.3em 0em darkgray;
    text-decoration: none;
    color: black;
  }

  info {
    flex-grow: 1;
  }
</style>
