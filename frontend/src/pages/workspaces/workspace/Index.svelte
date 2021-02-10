<script lang="ts">
    import { leftover } from "@roxi/routify"; 
    import { url } from "@roxi/routify";
    import WorkspaceInfo from "../../_components/Information.svelte";
    import Titlebar from "../../_components/Titlebar.svelte";
    import Container from "../../_components/Container.svelte";
    
    $: [workspace_id] = $leftover.split("/")
    let workspace = {
      workspace_id: workspace_id,
      name: "Introduction to testing",
      info: "This is an example of speech.This is an example of speech.This is an example of speech.",

      moderators: [ 
        { user_id: "klasa", name: "Klas Testsson"},
        { user_id: "jerry", name: "Jerry Eriksson"} 
      ],
      creator: { user_id: "admin", name: "Admin Adminsson" }
    };
</script>

<main>
  <Titlebar username="Kuba" />
  <test>
    <Container title="{workspace.name}">
      <WorkspaceInfo {...workspace}/>
    </Container>
    <a href={$url('../')}>Back</a>
  </test>
</main>


<style>
  test {
    max-width: 64em;
    display: block;
    margin-left: auto;
    margin-right: auto;
  }

</style>

