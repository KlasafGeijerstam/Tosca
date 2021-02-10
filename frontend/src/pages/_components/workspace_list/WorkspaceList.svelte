<script lang="ts">
  import WorkspaceItem from "./WorkspaceItem.svelte";
  import { Plus } from "tabler-icons-svelte";

  let workspaces = [
    { workspace_id: "1",
      name: "Workspace 1",
      info: "This is workspace 1. Blah blah blah.",
      active_list: "https://google.com",
    },
    { workspace_id: "2",
      name: "Workspace 2",
      info: "This is workspace 2. Blah blah blah."
    }];

</script>

<list>
  {#each workspaces as workspace }
      <WorkspaceItem {...workspace}/>
  {/each}

  <newWorkSpace>
    <button>
      <Plus />
      <h2>New Workspace</h2>
    </button>
  </newWorkSpace>
</list>

<style>
  list {
    display: flex;
    flex-direction: column;
    margin: 1em;
  }

  button {
    display: inline-flex;
    background-color: var(--background-light);
    margin-bottom: 1em;
    padding: 0.3em;
    align-items: center;
    border: 0cm;
    box-shadow: 0.1em 0.2em 0.3em 0em darkgray;
    color: white;
  }

  button > * {
    display: inline;
  }
</style>
