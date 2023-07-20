import { writable } from "svelte/store";

type UserStatusStore = {
  isSignedIn: boolean;
}

export const userStatus = writable<UserStatusStore>({ isSignedIn: false })
