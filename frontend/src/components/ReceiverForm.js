import "./ReceiverForm.css";

import React from "react";
import Button from "./Button";
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer, toast } from 'react-toastify';
import Loader from "./Loader";
import axios from "axios";

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

		let addr = "pylon-web-osl65qagha-uc.a.run.app";

		await axios({
			method: "POST",
			url: `https://${addr}:8080/receive`,
			timeout: 1000 * 30,
			headers: {
				"Content-Type": "application/json",
			},
			data: {
				code,
			},
		}).then(resp => {
			if (resp.status !== 200) {
				toast.error("Receiving message failed");
			} else {
				toast.success("Message received successfully");
				setMessage(resp.data.data.message);
				setTime(epochToDate(resp.data.data.time.secs_since_epoch));
			}
		}).catch(() => {
			toast.error("Receiving message timed out");
		})

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
			<h6 className="ReceiverForm-timestamp">Message sent at: {time || ""}</h6>
			<ToastContainer />
		</div>
	)
}

export default ReceiverForm;
