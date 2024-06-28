<script lang="ts">
  // import
  import Header from "./Header.svelte";
  import ThreadItemView from "./ThreadItemView.svelte";

  // props
  export let onSelectionChanged: (threadName: string) => void;

</script>

<div class="side-nav">
  <Header />
  <ThreadItemView threadName={"test"} onSelect={onSelectionChanged} />
  <ThreadItemView threadName={"test2"} onSelect={onSelectionChanged} />
  <ThreadItemView threadName={"private"} onSelect={onSelectionChanged} />

</div>

<style>
  .side-nav {
    min-width: 200px;
    padding-right: 2em;
  }
</style>