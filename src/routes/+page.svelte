<script lang="ts">
    let messages: String[] = [];

</script>

<ol>
    {#each messages as message}
        <li>{message}</li>
    {/each}
</ol>

<style>

</style>