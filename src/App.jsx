import {invoke} from "@tauri-apps/api/tauri";
import {sendNotification} from "@tauri-apps/api/notification";
import {useEffect} from "react";
import {TitleBar} from "./component/TitleBar.jsx";
import {Button, Input, useDisclosure} from "@nextui-org/react";
import CodeMirror from '@uiw/react-codemirror';
import {html} from "@codemirror/lang-html";
import {css} from "@codemirror/lang-css";
import "./App.css";
import {vscodeDark} from "@uiw/codemirror-theme-vscode";
import {Configuration} from "./component/Configuration.jsx";

function App() {
    const {isOpen, onOpen, onClose} = useDisclosure()
    useEffect(() => {
        init_info().catch(e => {
            sendNotification(`初始化配置信息错误:${e}`)
        })
    }, [])
    const init_info = async () => {
        await invoke("init_info")
    }
    return (
        <>
            <Configuration isOpen={isOpen} onClose={onClose}> < /Configuration>
            <TitleBar></TitleBar>
            <div className={'flex justify-around p-3'}>
                <div className={'flex items-center'}>
                    <span className={'w-20'}>主题:</span>
                    <Input size={"sm"} variant={"underlined"} placeholder={"主题"} color={"primary"}></Input>
                </div>
                <div className={'flex items-center'}>
                    <span className={'w-20'}>收件人:</span>
                    <Input size={"sm"} variant={"underlined"} placeholder={"收件人"} color={"primary"}></Input>
                </div>
            </div>
            <div className={'xl:flex'}>
                <CodeMirror
                    className={"xl:w-1/2"}
                    height={'calc(100vh - 130px)'}
                    extensions={[html(), css()]}
                    theme={vscodeDark}
                ></CodeMirror>
                <div className={'xl:w-1/2 xl:ml-2 hidden'} style={{height: 'calc(100vh - 130px)'}}>

                </div>
            </div>
            <div className={'flex justify-around'}>
                <Button size={"sm"} color={"default"} className={'mt-1.5'} onClick={() => onOpen()}>配置</Button>
                <Button size={"sm"} color={"secondary"} className={'mt-1.5'}>模板</Button>
                <Button size={"sm"} color={"primary"} className={'mt-1.5'}>Send</Button>
            </div>
        </>
    );
}

export default App;
