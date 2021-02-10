<script lang="ts">
  import { Plus } from "tabler-icons-svelte";
</script>

<div>
  <info>
      <h4>Name:</h4> 
        <p>Intro</p>
      <h4>Info:</h4> 
        <p>bla bla</p>
      <h4>Workspace ID:</h4> 
        <p>1337</p>
    </info>

  <moderators>
    <h4>Moderators:</h4>
    <ul>
      <li>Jerry</li>
      <li>Klas</li>
      <li>Kuba</li>
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
