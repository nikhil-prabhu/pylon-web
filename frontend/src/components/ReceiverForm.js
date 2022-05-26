import "./ReceiverForm.css";

import React from "react";
import Button from "./Button";
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer, toast } from 'react-toastify';
import Loader from "./Loader";

function ReceiverForm(props) {
	const [code, setCode] = React.useState();
	const [message, setMessage] = React.useState();
	const [time, setTime] = React.useState();
	const [inProgress, setInProgress] = React.useState();

	const epochToDate = (epochSecs) => {
		let date = new Date(0);
		date.setUTCSeconds(epochSecs);

		return date.toLocaleString();
	}

	const gotCode = (e) => {
		setCode(e.target.value);
	}

	const receiveMessage = async () => {
		setMessage(null);
		setInProgress(true);

		let resp = await fetch("http://localhost:8000/receive", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ code: code }),
		});

		if (resp.status !== 200) {
			toast.error("Failed to receive message");
		} else {
			let respJson = await resp.json();
			toast.success("Message received successfully");
			setMessage(respJson.data.message);
			setTime(epochToDate(respJson.data.time.secs_since_epoch));
		}

		setInProgress(false);
	}

	if (!props.show) {
		return null;
	}

	if (inProgress) {
		return (
			<Loader text={"Receiving"} />
		)
	}

	return (
		<div className="ReceiverForm">
			<h4 className="ReceiverForm-label">Code:</h4>
			<input className="ReceiverForm-code" onChange={gotCode} />
			<Button text={"Receive"} onClick={receiveMessage} disabled={!code} />

			<h4 className="ReceiverForm-label">Message:</h4>
			<div className="ReceiverForm-message">{message}</div>
			<h6 className="ReceiverForm-timestamp">Received at: {time || ""}</h6>
			<ToastContainer />
		</div>
	)
}

export default ReceiverForm;
