<script lang="ts">
  import Titlebar from "./Titlebar.svelte";
  import Container from "./Container.svelte";
  import WorkspacesList from "./WorkspacesList.svelte";
  import Information from "./Information.svelte";
</script>

<main>
  <Titlebar username="Kuba" />
  <test>
    <Container title="Information">
      <Information />
    </Container>
    <Container title="Workspaces">
      <WorkspacesList />
    </Container>
  </test>
</main>


<style>
  test {
    max-width: 64rem;
    display: block;
    margin-left: auto;
    margin-right: auto;
  }


  Container {
    max-width: 64rem;
  }
</style>
