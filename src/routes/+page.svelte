<script>
  import Header from "./components/Header.svelte";
  import SideNavigation from "./components/SideNavigation.svelte";
  import FloatingIconButton from "./components/FloatingIconButton.svelte";
  import MemoView from "./components/MemoView.svelte";

  const items = [
    { content: "hello svelte!", },
    { content: "this is a card",},
    { content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
  ];
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
    {#each items as item}
      <MemoView content={item.content}/>
    {/each}
  </div>
  <div class="icon-button">
    <FloatingIconButton />
  </div>
</div>

<style>
  :global(body) {
    background-color: #1f2733;
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