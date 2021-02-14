<script lang="ts">
  export let handler;
  export let title: string;
  export let color: string = "var(--background-light)";
  $: button_color = 'background-color: ' + color + ';';

</script>
<button on:click={handler} {title} style={button_color}>
  <slot></slot>
</button>

<style>
  button {
    display: inline-flex;
    background-color: var(--background-light);
    padding: 0.3em;
    align-items: center;
    border: 0cm;
    box-shadow: 0.1em 0.2em 0.3em 0em darkgray;
    color: white;
    cursor: pointer;
  }
</style>
