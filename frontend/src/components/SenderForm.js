import "./SenderForm.css";

import axios from "axios";

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

		let addr = "pylon-web-osl65qagha-uc.a.run.app";

		await axios({
			method: "GET",
			url: `https://${addr}:443/code`,
			timeout: 1000 * 30,
		}).then(resp => {
			if (resp.status !== 200) {
				toast.error("Failed to generate code");
				setCode(null);
			} else {
				setCode(resp.data.data);
				copyToClipboard(resp.data.data).catch(() => {
					toast.error("Failed to copy code to clipboard");
				});
				toast.info("Code copied to clipboard");
			}
		}).catch(() => {
			toast.error("Generating code timed out");
			setCode(null);
		});
	}

	const sendMessage = async () => {
		setInProgress(true);

		let addr = "pylon-web-osl65qagha-uc.a.run.app";

		await axios({
			method: "POST",
			url: `https://${addr}:443/send`,
			timeout: 1000 * 30,
			headers: {
				"Content-Type": "application/json",
			},
			data: {
				code,
				message,
			},
		}).then(resp => {
			if (resp.status !== 200) {
				toast.error("Receiving message failed");
			} else {
				toast.success("Message sent successfully");
			}
		}).catch(() => {
			toast.error("Sending message timed out");
		})

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
					<textarea className="SenderForm-message" rows={4} cols={50} onChange={getMessage} disabled={!code || (code === "Generating...")} />
					<Button text={"Send"} onClick={sendMessage} disabled={!code || (code === "Generating...")} />
				</div>
			}
			<ToastContainer />
		</div >
	)
}

export default SenderForm;
