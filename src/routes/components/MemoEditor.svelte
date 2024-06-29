<script lang="ts">
  // import
  import IconButton from "$lib/components/IconButton.svelte";

  // props
  export let dialog: HTMLDialogElement;
  export let thread: string;
  export let content: string;
  export let onSave: (thread: string, content: string) => void;

  const handleClose = () => {
    dialog.close();
  };

  const handleSave = () => {
    onSave(thread, content);
    dialog.close();
  };
</script>

<dialog bind:this={dialog} class="memo-editor">
  <div class="dialog-header">
    <div class="thread-editor">
      <span class="thread-editor-label">Thread: </span>
      <input bind:value={thread} />
    </div>
    <IconButton iconName={"close"} onClick={handleClose} />
  </div>
  <form method="dialog">
    <textarea bind:value={content} />
    <div class="dialog-footer">
      <button type="button" class="cancel-button" on:click={handleClose}>Cancel</button>
      <button type="submit" class="save-button" on:click={handleSave}>Save</button>
    </div>
  </form>
</dialog>

<style>
  .memo-editor {
    width: 600px;
    background-color: #1f2733;
    border: none;
    border-radius: 5px;
  }
  ::backdrop {
    background-color: #000000;
    opacity: 0.5;
  }

  .thread-editor-label {
    color: #5d7599;
    user-select: none;
  }
  input {
    font-family: 'Meiryo', sans-serif;
    font-size: medium;
    color: #d6d7d8;
    background-color: #1f2733;
    border: none;
    outline: none;
  }

  textarea {
    width: 100%;
    height: 200px;
    resize: none;
    font-family: 'Meiryo', sans-serif;
    font-size: large;
    color: #d6d7d8;
    background-color: #1f2733;
    border: none;
    outline: none;
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    padding: 0.4em 0em 1em 0em;
  }
  .dialog-footer {
    text-align: right;
  }

  .save-button {
    color: #d6d7d8;
    background-color: #e66077;
    font-size: large;
    border: none;
    border-radius: 5px;
    padding: 0.4em 1em;
    margin-left: 0.5em;
  }
  .save-button:hover {
    cursor: pointer;
  }
  .cancel-button {
    color: #5d7599;
    background-color: #1f2733;
    font-size: large;
    border: 1px solid #5d7599;
    border-radius: 5px;
    padding: 0.4em 1em;
    margin-left: 0.5em;
  }
  .cancel-button:hover {
    cursor: pointer;
  }
</style>