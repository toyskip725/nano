<script lang="ts">
  // import
  import MemoEditor from "./MemoEditor.svelte";

  let dialog: HTMLDialogElement;
  const onAdd = () => {
    dialog.showModal();
  };
</script>

<button type="button" class="floating-icon-button" on:click={onAdd}>
  <span class="button-icon material-symbols-outlined">add</span>
</button>
<MemoEditor bind:dialog content={""} onSave={(value) => console.log(value)} />

<style>
  .floating-icon-button {
    width: 5em;
    height: 5em;
    background-color: #e66077;
    border: none;
    border-radius: 50%;
  }
  .floating-icon-button:hover {
    cursor: pointer;
  }
  .button-icon {
    color: #d6d7d8;
  }
</style>