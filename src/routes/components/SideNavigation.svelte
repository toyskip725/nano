<script lang="ts">
  // import
  import Header from "./Header.svelte";
  import ThreadItemView from "./ThreadItemView.svelte";
  import type { ThreadInfo } from "$lib/types/threadInfo";
  import type { Memo } from "$lib/types/memo";

  // props
  export let items: Memo[];
  export let onSelectionChanged: (threadName: string) => void;

  let threads: ThreadInfo[];
  $: {
    threads = [];
    items.forEach((item) => {
      const target = threads.find(thread => thread.name === item.thread);
      if (target !== undefined) {
        target.count += 1;
        target.latestId = Math.max(target.latestId, item.id);
        return;
      }

      threads.push({
        name: item.thread,
        count: 1,
        latestId: item.id
      });
    });
    threads = threads.sort((a, b) => b.latestId - a.latestId);
  }
</script>

<div class="side-nav">
  <Header />
  {#each threads as thread}
    <ThreadItemView threadName={thread.name} count={thread.count} onSelect={onSelectionChanged} />
  {/each}
</div>

<style>
  .side-nav {
    min-width: 200px;
    padding-right: 2em;
  }
</style>