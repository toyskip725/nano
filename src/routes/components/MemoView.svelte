<script lang="ts">
  // import
  import MemoEditor from "./MemoEditor.svelte";
  
  // props
  export let content;

  let dialog: HTMLDialogElement;
  const onClickEdit = () => {
    dialog.showModal();
  };

  const onClickDelete = () => {
    alert("delete clicked");
  };
</script>

<div class="memo">
  <p>{content}</p>
  <div class="info-wrapper">
    <span class="info">2024/06/xx 12:00</span>
    <div class="info-button-wrapper">
      <button type="button" class="info-button" on:click={onClickEdit}>
        <span class="info-button-icon material-symbols-outlined">edit</span>
      </button>
      <button type="button" class="info-button" on:click={onClickDelete}>
        <span class="info-button-icon material-symbols-outlined">delete</span>
      </button>
    </div>
  </div>
</div>
<MemoEditor bind:dialog content={content} onSave={(value) => console.log(value)} />

<style>
  p {
    color: #d6d7d8;
  }

  .memo {
    max-width: 560px;
    min-width: 300px;
    border-bottom: 1px solid #3e4e66;
    padding: 1em;
    margin-left: 2em;
    margin-right: 2em;
  }

  .info-wrapper {
    display: flex;
  }
  .info {
    color: #5d7599;
    font-size: smaller;
  }
  .info-button {
    background-color: #1f2733;
    border: none;
  }
  .info-button:hover {
    cursor: pointer;
  }
  .info-button-icon {
    color: #5d7599;
    font-size: large;
  }
  .info-button-icon:hover {
    color: #e66077;
  }
  .info-button-wrapper {
    margin-left: auto;
  }
</style>