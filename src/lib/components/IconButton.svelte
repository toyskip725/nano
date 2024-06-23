<script lang="ts">
  export let iconName: string;
  export let onClick: () => void;
</script>

<button type="button" class="icon-button" on:click={onClick}>
  <span class="icon material-symbols-outlined">{iconName}</span>
</button>

<style>
.icon-button {
    background-color: #1f2733;
    border: none;
  }
  .icon-button:hover {
    cursor: pointer;
  }
  .icon {
    color: #5d7599;
    font-size: large;
  }
  .icon:hover {
    color: #e66077;
  }
</style>