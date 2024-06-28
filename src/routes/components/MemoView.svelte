<script lang="ts">
  // import
  import type { Memo } from "$lib/types/memo";
  import IconButton from "$lib/components/IconButton.svelte";
  import MemoEditor from "./MemoEditor.svelte";
  
  // props
  export let memo: Memo;
  export let onEdit: (id: number, thread: string, content: string) => void;
  export let onDelete: (id: number) => void;

  let dialog: HTMLDialogElement;
  const onClickEdit = () => {
    dialog.showModal();
  };

  const onClickDelete = () => {
    onDelete(memo.id);
  };
</script>

<div class="memo">
  <p>{memo.content}</p>
  <div class="info-wrapper">
    <span class="info">{memo.createdAt}</span>
    <div class="info-button-wrapper">
      <IconButton iconName={"edit"} onClick={onClickEdit} />
      <IconButton iconName={"delete"} onClick={onClickDelete} />
    </div>
  </div>
</div>
<MemoEditor bind:dialog thread={memo.thread} content={memo.content} onSave={(thread, content) => onEdit(memo.id, thread, content)} />

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
  .info-button-wrapper {
    margin-left: auto;
  }
</style>