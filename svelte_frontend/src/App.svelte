<script lang="ts">
  import Titlebar from "./Titlebar.svelte";
  import Container from "./Container.svelte";
  import ItemList from "./ItemList.svelte";
  import Information from "./Information.svelte";
</script>

<main>
  <Titlebar username="Kuba" />
  <Container title="Information">
    <Information />
  </Container>
  <Container title="Workspaces">
    <ItemList />
  </Container>
</main>
