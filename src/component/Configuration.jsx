import {Button, Input, Modal, ModalContent, ModalFooter} from "@nextui-org/react";
import {invoke} from "@tauri-apps/api/tauri";
import {useEffect, useState} from "react";
import {sendNotification} from "@tauri-apps/api/notification";

export const Configuration = ({isOpen, onClose}) => {
    const [confData, setConfData] = useState({
        smtpServer: "",
        username: "",
        password: ""
    })
    useEffect(() => {
        invoke("get_yaml_init").then((res) => {
            setConfData({
                smtpServer: res['smtp_service'],
                username: res['username'],
                password: res['password']
            })
        }).catch(e => {
            sendNotification(`初始化smtp错误:,${e}`)
        })
    }, [isOpen])
    const saveBtn = () => {
        invoke("save_email_info", confData).then(() => {
            sendNotification(`保存成功`)
            onClose()
        }).catch(e => {
            sendNotification(e)
        })
    }
    return (
        <Modal isOpen={isOpen}
               className={'pl-3 pt-5 pr-3 w-1/2'}
               hideCloseButton
               onClose={onClose}
               isDismissable={false}
               isKeyboardDismissDisabled={false}>
            <ModalContent>
                {
                    (onClose) => (
                        <>
                            <div className={'flex items-center mt-5'}>
                                <p className={'w-20'}>SMTP:</p>
                                <Input size={"sm"} className={'pl-2'} placeholder={'smtp服务器'}
                                       value={confData.smtpServer}
                                       onChange={(e) => {
                                           setConfData({
                                               ...confData,
                                               smtpServer: e.target.value
                                           })
                                       }}
                                ></Input>
                            </div>
                            <div className={'flex items-center mt-5'}>
                                <p className={'w-20'}>用户名:</p>
                                <Input size={"sm"} className={'pl-2'} placeholder={'smtp用户名'}
                                       value={confData.username}
                                       onChange={(e) => {
                                           setConfData({
                                               ...confData,
                                               username: e.target.value
                                           })
                                       }}
                                ></Input>
                            </div>
                            <div className={'flex items-center mt-5'}>
                                <p className={'w-20'}>密码:</p>
                                <Input type={'password'} size={"sm"} className={'pl-2'}
                                       value={confData.password}
                                       onChange={(e) => {
                                           setConfData({
                                               ...confData,
                                               password: e.target.value
                                           })
                                       }}
                                       placeholder={'smtp密码'}></Input>
                            </div>
                            <ModalFooter className={'mt-2'}>
                                <Button size={"sm"} onClick={() => onClose()}>取消</Button>
                                <Button size={"sm"} color={"primary"}
                                        className={'ml-3'}
                                        onClick={saveBtn}
                                >保存</Button>
                            </ModalFooter>
                        </>
                    )
                }
            </ModalContent>
        </Modal>
    )
}