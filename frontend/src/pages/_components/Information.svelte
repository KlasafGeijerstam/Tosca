<script lang="ts">
  import { Plus } from "tabler-icons-svelte";
  
  export let workspace_id: string;
  export let name: string;
  export let info: string;

  export let moderators;

  export let creator;
</script>

<div>
  <info>
      <h4>Name:</h4> 
        <p>{name}</p>
      <h4>Creator:</h4> 
        <p>{creator.user_id}</p>
      <h4>Information:</h4> 
        <p>{info}</p>
      <h4>Workspace ID:</h4> 
        <p>{workspace_id}</p>
    </info>

  <moderators>
    <h4>Moderators:</h4>
    <ul>
      {#each moderators as { user_id, name }}
        <li>{user_id} - {name}</li>
      {/each}
    </ul>
    <input type="text" placeholder="user id..">
  </moderators>
</div>

<style>
    div {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      padding: 1em;
    }

    input {
      width: 100%;
    }

    h4 {
      font-weight: bold;
    }
</style>
