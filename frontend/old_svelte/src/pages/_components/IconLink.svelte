<script lang="ts">
  import { AlertCircle } from 'tabler-icons-svelte';
  export let link: string;    
  export let title: string;
</script>

<a href={link} title={title}>
  <slot>
      <AlertCircle/>
  </slot>
</a>

<style>
  a {
    text-decoration: none;  
    color: black;
  }

  a:hover {
    text-decoration: underline;
  }
</style>
