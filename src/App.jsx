import {invoke} from "@tauri-apps/api/tauri";
import {sendNotification} from "@tauri-apps/api/notification";
import {useEffect, useState} from "react";
import {TitleBar} from "./component/TitleBar.jsx";
import {Button, Input, useDisclosure} from "@nextui-org/react";
import CodeMirror from '@uiw/react-codemirror';
import {html} from "@codemirror/lang-html";
import {css} from "@codemirror/lang-css";
import {vscodeDark} from "@uiw/codemirror-theme-vscode";
import {Configuration} from "./component/Configuration.jsx";
import "./App.css";

function App() {
    const {isOpen, onOpen, onClose} = useDisclosure()
    const [codeValue, setCodeValue] = useState('')
    const [emailData, setEmailData] = useState({
        toAddress: "",
        subject: "",
    })
    const [isLoading, setIsLoading] = useState(false)
    useEffect(() => {
        init_info().catch(e => {
            sendNotification(`初始化配置信息错误:${e}`)
        })
    }, [])
    const init_info = async () => {
        await invoke("init_info")
    }
    const sendBtn = () => {
        setIsLoading(true)
        invoke("send_email", {...emailData, body: codeValue}).then(r => {
            sendNotification("发送成功")
            setIsLoading(false)
        }).catch(e => {
            sendNotification(e)
            setIsLoading(false)
        })
    }
    return (
        <>
            <Configuration isOpen={isOpen} onClose={onClose}> < /Configuration>
            <TitleBar></TitleBar>
            <div className={'flex justify-around p-3'}>
                <div className={'flex items-center'}>
                    <span className={'w-20'}>主题:</span>
                    <Input size={"sm"} variant={"underlined"} placeholder={"主题"} color={"primary"}
                           value={emailData.subject}
                           onChange={(e) => {
                               setEmailData({...emailData, subject: e.target.value})
                           }}
                    ></Input>
                </div>
                <div className={'flex items-center'}>
                    <span className={'w-20'}>收件人:</span>
                    <Input size={"sm"} variant={"underlined"} placeholder={"收件人"} color={"primary"}
                           value={emailData.toAddress}
                           onChange={(e) => {
                               setEmailData({...emailData, toAddress: e.target.value})
                           }}
                    ></Input>
                </div>
            </div>
            <div className={'xl:flex'}>
                <CodeMirror
                    value={codeValue}
                    onChange={(e) => {
                        setCodeValue(e)
                    }}
                    className={"xl:w-1/2"}
                    height={'calc(100vh - 130px)'}
                    extensions={[html(), css()]}
                    theme={vscodeDark}
                ></CodeMirror>
                <div className={'xl:w-1/2 xl:ml-2 hidden xl:block'}
                     dangerouslySetInnerHTML={{__html: codeValue}}
                     style={{
                         height: 'calc(100vh - 130px)',
                         color: "#000",
                     }}>

                </div>
            </div>
            <div className={'flex justify-around'}>
                <Button size={"sm"} color={"default"} className={'mt-1.5'} onClick={() => onOpen()}>SMTP配置</Button>
                <Button size={"sm"} color={"secondary"} className={'mt-1.5'}>模板</Button>
                <Button size={"sm"} color={"primary"} className={'mt-1.5'} onClick={sendBtn}
                        isLoading={isLoading}>发送</Button>
            </div>
        </>
    );
}

export default App;
