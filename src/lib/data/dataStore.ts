import type { Memo } from "$lib/types/memo";

export const initializeStore = (): Memo[] => {
  return [
    {
      id: 1,
      thread: "test",
      createdAt: "2024/06/xx 12:00",
      content: "Hello Tauri + Sveltekit!"
    },
    {
      id: 2,
      thread: "test",
      createdAt: "2024/06/xx 12:00",
      content: "This is a Memo"
    },
    {
      id: 3,
      thread: "test2",
      createdAt: "2024/06/xx 12:00",
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. "
    },
    {
      id: 4,
      thread: "test",
      createdAt: "2024/06/xx 12:00",
      content: "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. ",
    },
    {
      id: 5,
      thread: "test2",
      createdAt: "2024/06/xx 12:00",
      content: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
    },
  ];
}
