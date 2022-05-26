import "./SenderForm.css";

import React from "react";
import Loader from "./Loader";
import Button from "./Button";
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer, toast } from 'react-toastify';

const copyToClipboard = str => {
	if (navigator && navigator.clipboard && navigator.clipboard.writeText)
		return navigator.clipboard.writeText(str);
	return Promise.reject("The Clipboard API is not available.");
};

function SenderForm(props) {
	const [code, setCode] = React.useState();
	const [message, setMessage] = React.useState();
	const [inProgress, setInProgress] = React.useState();

	const getMessage = (e) => {
		setMessage(e.target.value);
	}

	const genCode = async () => {
		setCode("Generating...");

		let resp = await fetch("http://localhost:8000/code", {
			method: "GET",
		});
		let respJson = await resp.json();
		setCode(respJson.data);
		copyToClipboard(respJson.data).catch(() => {
			toast.error("Failed to copy code to clipboard");
		});
		toast.info("Code copied to clipboard");
	}

	const sendMessage = async () => {
		setInProgress(true);

		let resp = await fetch("http://localhost:8000/send", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ code: code, message: message }),
		});

		if (resp.status !== 200) {
			toast.error("Failed to send message");
		} else {
			toast.success("Message sent successfully");
		}

		setCode(null);
		setInProgress(false);
	}

	if (!props.show) {
		return null;
	}

	return (
		<div className="SenderForm">
			<h4 className="SenderForm-label">Code:</h4>
			<div className="SenderForm-code">{code || "-"}</div>
			<Button text={"Generate"} onClick={genCode} disabled={code} />

			{inProgress ?
				<Loader text={"Sending"} />
				: <div>
					<h4 className="SenderForm-label">Message:</h4>
					<textarea className="SenderForm-message" rows={4} cols={50} onChange={getMessage} disabled={!code} />
					<Button text={"Send"} onClick={sendMessage} disabled={!code} />
				</div>
			}
			<ToastContainer />
		</div >
	)
}

export default SenderForm;
