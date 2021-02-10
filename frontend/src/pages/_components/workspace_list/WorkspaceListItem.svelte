<script lang="ts">
    import { url } from "@roxi/routify";
    export let workspace_id: string;
    export let name: string;
    export let info: string;
</script>

<div>
    <a href={$url() + '/' + workspace_id}>
        <span>Id: {workspace_id}</span>
        <span>Name: {name}</span>
        <span>Info: {info}</span>
    </a>
</div>
