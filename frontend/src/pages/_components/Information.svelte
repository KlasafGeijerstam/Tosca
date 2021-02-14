<script lang="ts">
  import { X, Trash, UserPlus, Check, Edit } from "tabler-icons-svelte";
  import IconButton from 'IconButton.svelte';  
  import InputWithButton from 'InputWithButton.svelte';
  import Button from "./Button.svelte";
  import EditableField from "./EditableField.svelte";
  export let workspace_id: string;
  export let name: string;
  export let info: string;

  export let moderators;

  export let creator;

  let edit_enabled = false;
  
  function toggle_edit() {
    edit_enabled = !edit_enabled;
  }

  function delete_moderator(moderator: string) {
    alert("Time to delete " + moderator);
  }

  function save() {
    alert("Saving!");
    edit_enabled = false;
  }

  function cancel() {
    edit_enabled = false;
  }
</script>

<div id="idbox">
  <div id="info">
    <EditableField title="Name" value={name} edit={edit_enabled}/>
    <EditableField title="Information" value={info} edit={edit_enabled}/>
    <EditableField title="Workspace ID" value={workspace_id} edit={edit_enabled}/>
    <EditableField title="Creator" value={creator.user_id}/>
  </div>

  <div id="moderators">
    <h4>Moderators:</h4>
    <ul>
        {#each moderators as { user_id, name }}
          {#if edit_enabled}
            <li>{user_id} - {name}
              <IconButton handler="{() => delete_moderator(user_id)}" title="Delete moderator">
                <Trash size="1em" color="red"/>
              </IconButton>
            </li>
          {:else}
              <li>{user_id} - {name}</li>
          {/if}
        {/each}
    </ul>
    {#if edit_enabled}
      <InputWithButton placeholder="user id.." handler="{(mod) => alert('Time to add ' + mod) }" title="Add moderator">
        <UserPlus color="white"/>
      </InputWithButton>
    {/if}
  </div>
  <div id="edit">
    {#if edit_enabled}
      <div>
        <Button color="var(--error)" handler={cancel}>
          <X/>
          Cancel
        </Button>
        <Button color="var(--success)" handler={save}>
          <Check/>
          Save
        </Button>
      </div>
    {:else}
      <Button handler={toggle_edit}>
        <Edit/>
        Edit
      </Button>
    {/if}
  </div>
</div>

<style>
    #edit {
      grid-column: 1/-1;  
      display: flex;
      justify-content: end;
    }

    #idbox {
      display: grid;
      /*TODO: FIX for smaller screens*/
      grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
      padding: 1em;
      row-gap: 1em;
      column-gap: 1em;
    }

</style>
