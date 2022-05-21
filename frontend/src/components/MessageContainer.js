import React from "react";
import { v4 as uuidv4 } from "uuid";

import "./Dashboard.css";
import Button from "./Button";

function MessageContainer() {
    const [msgList , setMessageList] = React.useState([]);
    const [message, setMessage] = React.useState('');

    function handleChange(e) {
        setMessage(e.target.value)
    }

    function sendMessage() {
        // send message
        

        // Add message to list and clear input
        const newMsgList = msgList.concat({ message, id: uuidv4() });
        setMessageList(newMsgList);
        setMessage('');
    }

    function receiveMessage() {
    }

    return(
        <div>
            <div className="Dashboard-buttons">
                <Button text="Send" onClick={sendMessage} />
                <Button text="Receive" onClick={receiveMessage} />
            </div>
            <div><input type="text" onChange={handleChange} value={message} /></div>
            <div className="Message-List">
                <ul>
                    {msgList.map((item) => (
                        <li key={item.id}>{item.message}</li>
                    ))}
                </ul>
            </div>
        </div>
    );
}

export default MessageContainer;