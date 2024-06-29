<script lang="ts">
  // props
  export let threadName = "";

  $: title = threadName !== "" ? `nano / ${threadName}` : "nano /";
</script>

<p class="title"><span class="title-icon">＊</span>{title}</p>

<style>
  .title {
    color: #d6d7d8;
    user-select: none;
    padding-top: 1em;
  }
  .title-icon {
    color: #e66077;
  }
</style>