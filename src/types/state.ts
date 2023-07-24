export interface GlobalAppState {
  count: number;
  check: boolean;
  setCount: (count: number) => void;
  setCheck: (check: boolean) => void;
}
