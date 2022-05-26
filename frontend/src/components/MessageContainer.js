import React from "react";
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer, toast } from 'react-toastify';


import "./Dashboard.css";
import Button from "./Button";

function MessageContainer() {
    const [message, setMessage] = React.useState('');
    const [codeTxt, setCode] = React.useState('');
    const [rcvMsg, setRcvMsg] = React.useState('');

    function handleChange(e) {
        setMessage(e.target.value)
    }

    function handleChangeCode(e) {
        setCode(e.target.value);
    }

    function handleChangeRcv(e) {
        e.preventDefault();
    }

    const getCode = async (callback) => {
        let resp = await fetch("http://localhost:8000/code", {
            method: "GET"
        });
        return await resp.json();
    }

    const send = async () => {
        let codeJSON = await getCode();

        setCode(codeJSON.data);
        toast.info("waiting for receiver");

        await fetch("http://localhost:8000/send", {
            method: "POST",
            body: JSON.stringify({code: codeJSON.data, message: message}),
            headers: {
                "Content-Type": "application/json"
            }
        })
        .then(respone => respone.json())
        .then(data => {
            setMessage("");
            setCode("");
            toast.success("Message Sent");
        })
        .catch((error) => {
            toast.error("error while sending message");
            console.error("error while sending message");
        });
    }

    function receiveMessage() {
        fetch("http://localhost:8000/receive", {
            method: "POST",
            body: JSON.stringify({code: codeTxt}),
            headers: {
                "Content-Type": "application/json"
            }
        })
        .then(respone => respone.json())
        .then(msg => {
            setRcvMsg(msg.data.message);
            toast.success("Message Received");
        })
        .catch((error) => {
            toast.error("Error receiving message");
            console.error("Error receiving message");
        });
    }

    return(
        <div>
            <div className="Dashboard-buttons">
                <Button text="Send" onClick={send} />
                <Button text="Receive" onClick={receiveMessage} />
            </div>
            <div>
                <label>Code: </label>
                <input type="text" onChange={handleChangeCode} value={codeTxt} />
                <br />
                <label>Message to send: </label>
                <input type="text" onChange={handleChange} value={message} />
                <br />
                <label>Received Message: </label>
                <input type="text" onChange={handleChangeRcv} value={rcvMsg} readOnly />
                <br />
            </div>
            <ToastContainer />
        </div>
    );
}

export default MessageContainer;