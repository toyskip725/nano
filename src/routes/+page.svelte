<script lang="ts">
  import SideNavigation from "./components/SideNavigation.svelte";
  import FloatingIconButton from "./components/FloatingIconButton.svelte";
  import MemoView from "./components/MemoView.svelte";
  import { initializeStore } from "$lib/data/dataStore";
  import { toNormalFormat } from "$lib/utils/datetimeFormat";

  let memoData = initializeStore();
  let thread = "";

  const onThreadSelectionChanged = (threadName: string) => {
    thread = threadName;
  };

  const onEdit = (id: number, thread: string, content: string) => {
    const targetIndex = memoData.findIndex(memo => memo.id === id);
    if (targetIndex === -1) {
      return;
    }

    const targetMemo = memoData[targetIndex];
    targetMemo.thread = thread;
    targetMemo.content = content;
    memoData = memoData.toSpliced(targetIndex, 1, targetMemo);
  };

  const onSave = (thread: string, content: string) => {
    const newMemo = {
      id: memoData.length + 1,
      thread: thread,
      createdAt: toNormalFormat(new Date()),
      content: content,
    };
    memoData = [...memoData, newMemo];
  };

  const onDelete = (id: number) => {
    memoData = memoData.filter(memo => memo.id !== id);
  };
</script>

<svelte:head>
  <link 
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,300,0,0" />
</svelte:head>

<div class="main">
  <div class="sidebar">
    <SideNavigation onSelectionChanged={onThreadSelectionChanged} />
  </div>
  <div class="memo-stack">
    {#each memoData.filter(memo => memo.thread === thread).toReversed() as memo}
      <MemoView memo={memo} onEdit={onEdit} onDelete={onDelete} />
    {/each}
  </div>
  <div class="icon-button">
    <FloatingIconButton onSave={onSave} />
  </div>
</div>

<style>
  :global(body) {
    background-color: #1f2733;
    font-family:'Meiryo', sans-serif;
  }

  .main {
    max-width: 960px;
    margin: auto;
    display: flex;
  }
  .sidebar {
    position: sticky;
    align-self: flex-start;
    top: 1em;
  }
  .icon-button {
    position: fixed;
    right: 2em;
    bottom: 2em;
  }

  @media screen and (min-width: 960px) {
    .icon-button {
      right: calc(50% - 480px);
    }
  }
</style>