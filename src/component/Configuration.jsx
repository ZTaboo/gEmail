import {Button, Input, Modal, ModalContent, ModalFooter} from "@nextui-org/react";
import {invoke} from "@tauri-apps/api/tauri";

export const Configuration = ({isOpen, onClose}) => {
    const saveBtn = async () => {
        let res = await invoke("save_email_info", {smtpServer: "server", username: "username", password: "password"})
        console.log("res", res)
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
                                <Input size={"sm"} className={'pl-2'} placeholder={'smtp服务器'}></Input>
                            </div>
                            <div className={'flex items-center mt-5'}>
                                <p className={'w-20'}>用户名:</p>
                                <Input size={"sm"} className={'pl-2'} placeholder={'smtp用户名'}></Input>
                            </div>
                            <div className={'flex items-center mt-5'}>
                                <p className={'w-20'}>密码:</p>
                                <Input type={'password'} size={"sm"} className={'pl-2'}
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