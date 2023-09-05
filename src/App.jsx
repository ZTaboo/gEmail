import {invoke} from "@tauri-apps/api/tauri";
import {sendNotification} from "@tauri-apps/api/notification";
import {Button} from "@nextui-org/react";
import {useEffect} from "react";
import "./App.css";
import {TitleBar} from "./component/TitleBar.jsx";

function App() {
    useEffect(() => {
        init_info().catch(e => {
            sendNotification(`初始化配置信息错误:${e}`)
        })
    }, [])
    const init_info = async () => {
        await invoke("init_info")
    }
    const zeroBtn = async () => {
        let name = await invoke("zero");
        sendNotification(name);
    };
    return (
        <>
            <TitleBar></TitleBar>
            <Button color="primary" onClick={zeroBtn}>
                Btn
            </Button>
        </>
    );
}

export default App;
