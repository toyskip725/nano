import { invoke } from "@tauri-apps/api";
import type { Memo } from "$lib/types/memo";

export namespace TauriCommand {
  export const invokeCreate = async (memo: Memo) => {
    await invoke("cmd_create", {
      id: memo.id,
      thread: memo.thread,
      createdAt: memo.createdAt,
      content: memo.content
    });
  }

  export const invokeGetAll = async () => {
    const result = await invoke("cmd_get_all");
    return result as Memo[];
  }

  export const invokeUpdate = async (memo: Memo) => {
    await invoke("cmd_update", {
      id: memo.id,
      thread: memo.thread,
      createdAt: memo.createdAt,
      content: memo.content
    });
  }

  export const invokeDelete = async (id: number) => {
    await invoke("cmd_delete", {
      id: id
    });
  }
}