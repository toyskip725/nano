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
}