import {appWindow} from "@tauri-apps/api/window";
import '../assets/css/titleBar.css'
import {useState} from "react";

export const TitleBar = () => {
    const [window, setWindow] = useState(false);

    return (
        <div data-tauri-drag-region className="titlebar">
            <div className="titlebar-button" onClick={() => appWindow.minimize()}>
                <img
                    src="https://api.iconify.design/mdi:window-minimize.svg?color=%236b717d"
                    alt="minimize"
                />
            </div>
            <div
                className="titlebar-button"
                onClick={() => {
                    console.log(window);
                    window ? appWindow.unmaximize() : appWindow.maximize();
                    setWindow(!window);
                }}
            >
                <img
                    src={
                        window
                            ? "https://api.iconify.design/clarity/window-restore-line.svg?color=%236b717d&width=20&height=24"
                            : "https://api.iconify.design/mdi:window-maximize.svg?color=%236b717d"
                    }
                    alt="maximize"
                />
            </div>
            <div className="titlebar-button" onClick={() => appWindow.close()}>
                <img
                    src="https://api.iconify.design/mdi:close.svg?color=%236b717d"
                    alt="close"
                />
            </div>
        </div>

    )
}