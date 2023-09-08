import {Button, Card, CardBody, Modal, ModalContent} from "@nextui-org/react";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";

export const TempModal = ({isOpen, onClose, useTemplate}) => {
    const [tpList, setTpList] = useState([])
    useEffect(() => {
        invoke("get_template").then(r => {
            setTpList(r)
        })
    }, [])
    return (
        <Modal
            className={'pl-3 pt-3 pr-3'}
            isOpen={isOpen}
            onClose={onClose}
            hideCloseButton
            isDismissable={true}
            size={"full"}
            isKeyboardDismissDisabled={false}
        >
            <ModalContent>
                <div className={'h-full'}>
                    <Button onClick={() => onClose()} variant={"light"} size={"sm"}>返回</Button>
                    {
                        tpList.map(item => (
                            /*<div key={item.id}>
                                <span>{item.title}</span>
                            </div>*/
                            <Card key={item.id} className={'bg-gray-700 mt-5'}>
                                <CardBody>
                                    <div className={'flex justify-between items-center'}>
                                        <span>{item.title}</span>
                                        <Button size={"sm"} variant={"light"} onClick={() => {
                                            useTemplate(item.content)
                                        }}>使用</Button>
                                    </div>
                                </CardBody>
                            </Card>
                        ))
                    }
                </div>
            </ModalContent>
        </Modal>
    )
}