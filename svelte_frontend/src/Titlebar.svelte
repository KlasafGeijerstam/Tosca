<script lang="ts">
  export let username: string;
</script>
<titlebar>
    <h1>Tosca</h1>
    <h2>{ username }</h2>
</titlebar>
<style>
    titlebar {
        display: flex;
        justify-content: space-between;
        align-items: baseline;
        background-color: var(--background-dark);
        color: white;
        box-shadow: 0em 0.3em 0.5em 0.1em darkgray;
    }
    
    titlebar > * {
        margin-left: 1em;
        margin-right: 1em;
    }

</style>
