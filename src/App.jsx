import {invoke} from "@tauri-apps/api/tauri";
import {sendNotification} from "@tauri-apps/api/notification";
import {useEffect, useState} from "react";
import {TitleBar} from "./component/TitleBar.jsx";
import {Button, Input, Modal, ModalContent, ModalFooter, Tooltip, useDisclosure} from "@nextui-org/react";
import CodeMirror from '@uiw/react-codemirror';
import {html} from "@codemirror/lang-html";
import {css} from "@codemirror/lang-css";
import {vscodeDark} from "@uiw/codemirror-theme-vscode";
import {Configuration} from "./component/Configuration.jsx";
import "./App.css";
import {TempModal} from "./component/TempModal.jsx";

function App() {
    const {isOpen, onOpen, onClose} = useDisclosure()
    const {isOpen: isOpen1, onOpen: onOpen1, onClose: onClose1} = useDisclosure()
    const {isOpen: isTpOpen, onOpen: onTpOpen, onClose: onTpClose} = useDisclosure()
    const [codeValue, setCodeValue] = useState('')
    const [emailData, setEmailData] = useState({
        toAddress: "",
        subject: "",
    })
    const [htmlTheme, setHtmlTheme] = useState("")
    const [isLoading, setIsLoading] = useState(false)
    const [tpName, setTpName] = useState('')
    useEffect(() => {
        init_info().catch(e => {
            sendNotification(`初始化配置信息错误:${e}`)
        })
    }, [])
    // 禁用右键
    window.addEventListener("contextmenu", (e) => e.preventDefault(), false);
    // 拦截没有target参数的跳转链接;
    document.addEventListener('click', (e) => {
        let target = e.target;
        // 检查a标签是否有target属性
        if (target.nodeName === 'A' && !target.target) {
            console.log(target.nodeName)
            e.preventDefault();
            e.stopPropagation();
        }
    });

    const init_info = async () => {
        await invoke("init_info")
    }
    const sendBtn = () => {
        if (emailData.toAddress.trim().length === 0 || emailData.subject.trim().length === 0 || codeValue.trim().length === 0) {
            sendNotification("请填写必要信息")
            return
        }
        setIsLoading(true)
        invoke("send_email", {...emailData, body: codeValue}).then(() => {
            sendNotification("发送成功")
            setIsLoading(false)
        }).catch(e => {
            sendNotification(e)
            setIsLoading(false)
        })
    }
    const useTemplate = (con) => {
        setCodeValue(con)
        onClose1()
    }

    const saveBtn = () => {
        invoke("save_template", {title: tpName, con: codeValue}).then(() => {
            onTpClose()
            sendNotification(`保存成功`)
        }).catch(e => {
            sendNotification(e)
        })
    }


    return (
        <>
            {/*smtp配置弹出层*/}
            <Configuration isOpen={isOpen} onClose={onClose}> < /Configuration>
            <TempModal isOpen={isOpen1} onClose={onClose1} useTemplate={useTemplate}></TempModal>
            <Modal onClose={onTpClose} isOpen={isTpOpen} hideCloseButton isKeyboardDismissDisabled
                   isDismissable={false}>
                <ModalContent>
                    <div className={'pt-5 pl-5 pr-5 pb-3'}>
                        <div className={'flex items-center'}>
                            <span className={'w-24'}>模板名称:</span>
                            <Input size={"sm"} placeholder={'请输入模板名称'} value={tpName} onChange={(e) => {
                                setTpName(e.target.value)
                            }}></Input>
                        </div>
                    </div>
                    <ModalFooter>
                        <Button size={"sm"} onClick={() => onTpClose()}>取消</Button>
                        <Button color={"primary"} size={"sm"} className={'ml-4'} onClick={saveBtn}>保存</Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>
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
                    className={"xl:w-1/2 relative"}
                    height={'calc(100vh - 130px)'}
                    extensions={[html(), css()]}
                    theme={vscodeDark}
                >
                    <Tooltip content={"另存为模板"} placement={'left'}>
                        <Button className={'absolute right-5 bottom-4'} style={{zIndex: 1}} isIconOnly={true}
                                onClick={() => onTpOpen()}>
                            <img src="https://api.iconify.design/teenyicons/save-solid.svg?color=white" alt=""/>
                        </Button>
                    </Tooltip>
                </CodeMirror>
                <div className={'xl:w-1/2 xl:ml-2 hidden xl:block overflow-auto'}
                     dangerouslySetInnerHTML={{__html: codeValue}}
                     style={{
                         height: 'calc(100vh - 130px)',
                         color: "#000",
                         backgroundColor: htmlTheme
                     }}>
                </div>
                <Tooltip content={"预览背景;"} showArrow placement={"left"}>
                    <Button isIconOnly={true}
                            className={htmlTheme === "white" ? 'absolute hidden xl:block right-5 bottom-14 light' : 'absolute hidden xl:block right-5 bottom-14 dark'}
                            onClick={() => {
                                htmlTheme.trim().length === 0 ? setHtmlTheme("white") : setHtmlTheme("")
                            }}>
                        <img
                            src={htmlTheme === "white" ? "https://api.iconify.design/ic/baseline-dark-mode.svg" : "https://api.iconify.design/ph/sun-fill.svg?color=white"}
                            className={'m-auto'}
                            alt=""/>
                    </Button>
                </Tooltip>
            </div>
            <div className={'flex justify-around'}>
                <Button size={"sm"} color={"default"} className={'mt-1.5'} onClick={() => onOpen()}>SMTP配置</Button>
                <Button size={"sm"} color={"secondary"} className={'mt-1.5'} onClick={() => onOpen1()}>模板</Button>
                <Button size={"sm"} color={"primary"} className={'mt-1.5'} onClick={sendBtn}
                        isLoading={isLoading}>发送</Button>
            </div>
        </>
    );
}

export default App;
