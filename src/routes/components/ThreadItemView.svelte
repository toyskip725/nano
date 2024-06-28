<script lang="ts">
  // props
  export let threadName: string;
  export let onSelect: (threadName: string) => void;

  const onClick = () => {
    onSelect(threadName);
  };
</script>

<div>
  <button type="button" class="thread-item" on:click={onClick}>{threadName}</button>
</div>

<style>
  .thread-item {
    width: 100%;
    color: #d6d7d8;
    background-color: #1f2733;
    font-size: medium;
    text-align: left;
    border: none;
    border-bottom: 1px solid #3e4e66;
    padding: 1em;
  }
  .thread-item:hover {
    cursor: pointer;
  }
</style>
