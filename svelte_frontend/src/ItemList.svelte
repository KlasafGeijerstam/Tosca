<script lang="ts">
  import Item from "./Item.svelte";
  import { Plus } from "tabler-icons-svelte";
</script>

<list>
  <Item courseName="Kurs 1" courseText="Dålig kurs" />
  <Item
    courseName="Kurs 2"
    courseText="There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc."
  />
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
