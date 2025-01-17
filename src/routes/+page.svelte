<script lang="ts">
  // import
  import SideNavigation from "./components/SideNavigation.svelte";
  import FloatingIconButton from "./components/FloatingIconButton.svelte";
  import MemoView from "./components/MemoView.svelte";
  import { toNormalFormat } from "$lib/utils/datetimeFormat";
  import { TauriCommand } from "$lib/command";
  import type { Memo } from "$lib/types/memo";

  let memoData: Memo[] = [];
  let thread = "";

  const sync = async () => {
    memoData = await TauriCommand.invokeGetAll();
  };
  sync();

  const onThreadSelectionChanged = (threadName: string) => {
    thread = threadName;
  };

  const onEdit = async (id: number, thread: string, content: string) => {
    const targetIndex = memoData.findIndex(memo => memo.id === id);
    if (targetIndex === -1) {
      return;
    }

    const targetMemo = memoData[targetIndex];
    targetMemo.thread = thread;
    targetMemo.content = content;

    await TauriCommand.invokeUpdate(targetMemo);
    await sync();
  };

  const onSave = async (thread: string, content: string) => {
    const latestId = memoData.length !== 0 ? memoData.toSorted((a, b) => b.id - a.id)[0].id : 0;
    const newMemo = {
      id: latestId + 1,
      thread: thread,
      createdAt: toNormalFormat(new Date()),
      content: content,
    };
    await TauriCommand.invokeCreate(newMemo);
    await sync();
  };

  const onDelete = async (id: number) => {
    await TauriCommand.invokeDelete(id);
    await sync();
  };
</script>

<svelte:head>
  <link 
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,300,0,0" />
</svelte:head>

<div class="main">
  <div class="sidebar">
    <SideNavigation items={memoData} onSelectionChanged={onThreadSelectionChanged} />
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
    top: 0em;
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