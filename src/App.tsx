import { useEffect } from "react";
import useGlobalAppStore, { subscribeStateSync } from "@states/global-state";

import "./App.css";

function App() {
  const { count, setCount, check, setCheck } = useGlobalAppStore();

  useEffect(() => {
    const unsubscribeStateSync = subscribeStateSync();
    return () => {
      unsubscribeStateSync.then((unsubscribe) => unsubscribe());
    };
  }, []);

  return (
    <div className="app_container">
      <div className="menu_container">
        <div className="menu_item">
          <h1 className="menu_heading">Count: {count}</h1>
          <div className="menu_button_container">
            <button
              className="menu_button count_btn"
              onClick={() => setCount(count + 1)}
            >
              Increment
            </button>
            <button
              className="menu_button count_btn"
              onClick={() => setCount(count - 1)}
            >
              Decrement
            </button>
          </div>
        </div>
        <div className="menu_item">
          <h1 className="menu_heading">Check: {check.toString()}</h1>
          <button className="menu_button" onClick={() => setCheck(!check)}>
            Toggle
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
