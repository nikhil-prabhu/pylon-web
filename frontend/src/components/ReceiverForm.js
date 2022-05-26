import "./ReceiverForm.css";

import React from "react";
import Button from "./Button";
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer, toast } from 'react-toastify';

function ReceiverForm(props) {
	const [code, setCode] = React.useState();
	const [message, setMessage] = React.useState();

	const gotCode = (e) => {
		setCode(e.target.value);
	}

	const receiveMessage = async () => {
		setMessage(null);
		let resp = await fetch("http://localhost:8000/receive", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ code: code }),
		});
		let respJson = await resp.json();

		if (resp.status !== 200) {
			toast.error("Failed to receive message");
		} else {
			toast.success("Message received successfully");
			setMessage(respJson.data.message);
		}
	}

	if (!props.show) {
		return null;
	}

	return (
		<div className="ReceiverForm">
			<h4 className="ReceiverForm-label">Code:</h4>
			<input className="ReceiverForm-code" onChange={gotCode} />
			<Button text={"Receive"} onClick={receiveMessage} disabled={!code} />

			<h4 className="ReceiverForm-label">Message:</h4>
			<div className="ReceiverForm-message">{message}</div>
			<ToastContainer />
		</div>
	)
}

export default ReceiverForm;
