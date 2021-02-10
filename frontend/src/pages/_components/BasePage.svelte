<script lang="ts">
    import Titlebar from "Titlebar.svelte";
    import Container from "Container.svelte";
</script>
<main>
  <Titlebar username="Kuba" />
  <test>
    <slot>
      <Container title="Empty page">
      </Container>
    </slot>
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

