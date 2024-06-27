<script lang="ts">
  import Header from "./components/Header.svelte";
  import SideNavigation from "./components/SideNavigation.svelte";
  import FloatingIconButton from "./components/FloatingIconButton.svelte";
  import MemoView from "./components/MemoView.svelte";
  import { initializeStore } from "$lib/data/dataStore";

  let memoData = initializeStore();
  const onSave = (thread: string, content: string) => {
    const newMemo = {
      id: memoData.length,
      thread: thread,
      createdAt: Date.now.toString(),
      content: content,
    };
    memoData = [...memoData, newMemo];
  }
</script>

<svelte:head>
  <link 
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,300,0,0" />
</svelte:head>

<header>
  <Header />
</header>
<div class="main">
  <SideNavigation />
  <div>
    {#each memoData.toReversed() as memo}
      <MemoView content={memo.content}/>
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

  header {
    max-width: 960px;
    margin: auto;
  }
  .main {
    max-width: 960px;
    margin: auto;
    display: flex;
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