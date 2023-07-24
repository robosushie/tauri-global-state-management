import { emit as emitStateEvent, listen } from "@tauri-apps/api/event";
import { GlobalAppState } from "@app/types/state";
import { create } from "zustand";

import { EVENTS, GLOBAL_APP_STATE_MACRO } from "@app/constants";

const emit = (key: number, value: any) => {
  console.log(Object.keys(value)[0]);
  emitStateEvent(EVENTS.STATE_CHANGE_EVENT, {
    key: key,
    value: value,
  });
};

const useGlobalAppStore = create<GlobalAppState>(() => ({
  count: 0,
  check: false,
  setCount: (count: number) => {
    emit(GLOBAL_APP_STATE_MACRO.COUNT, count);
  },
  setCheck: (check: boolean) => {
    emit(GLOBAL_APP_STATE_MACRO.CHECK, check);
  },
}));

const getKey = (key: number) => {
  if (key == GLOBAL_APP_STATE_MACRO.COUNT) {
    return "count";
  } else if (key == GLOBAL_APP_STATE_MACRO.CHECK) {
    return "check";
  }
};

const subscribeStateSync = async () => {
  const unsubscribeStateSyncListener = await listen(
    EVENTS.STATE_SYNC_EVENT,
    (event) => {
      const key = (event.payload as any).key;
      const value = (event.payload as any).value;
      console.log(event);
      useGlobalAppStore.setState({ [getKey(key) as string]: value });
    }
  );

  return async () => {
    unsubscribeStateSyncListener();
  };
};

export default useGlobalAppStore;
export { subscribeStateSync };
