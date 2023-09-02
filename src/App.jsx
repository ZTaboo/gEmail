import { invoke } from "@tauri-apps/api/tauri";
import { sendNotification } from "@tauri-apps/api/notification";
import { Button } from "@nextui-org/react";
import "./App.css";

function App() {
  const zeroBtn = async () => {
    let name = await invoke("zero");
    sendNotification(name);
  };

  return (
    <>
      <div data-tauri-drag-region class="titlebar1">
        <div class="titlebar-button" id="titlebar-minimize">
          <img
            src="https://api.iconify.design/mdi:window-minimize.svg?color=%236b717d"
            alt="minimize"
          />
        </div>
        <div class="titlebar-button" id="titlebar-maximize">
          <img
            src="https://api.iconify.design/mdi:window-maximize.svg?color=%236b717d"
            alt="maximize"
          />
        </div>
        <div class="titlebar-button" id="titlebar-close">
          <img src="https://api.iconify.design/mdi:close.svg?color=%236b717d" alt="close" />
        </div>
      </div>
      <Button color="primary" onClick={zeroBtn}>
        Btn
      </Button>
    </>
  );
}

export default App;
