import "./Dashboard.css";

import React from "react";
import Button from "./Button";
import SenderForm from "./SenderForm";
import ReceiverForm from "./ReceiverForm";

function Dashboard(props) {
	const [currentView, setCurrentView] = React.useState();

	const setSendView = () => {
		setCurrentView("send");
	}

	const setReceiveView = () => {
		setCurrentView("receive");
	}

	return (
		<div className="Dashboard">
			<h1 className="Dashboard-header">{props.name || "Dashboard"}</h1>
			<img className="Dashboard-logo" src={props.logo || ""} alt="logo" />

			<div className="Dashboard-buttons">
				<Button text="Send" onClick={setSendView} />
				<Button text="Receive" onClick={setReceiveView} />
			</div>

			<SenderForm show={currentView === "send"} />
			<ReceiverForm show={currentView === "receive"} />
		</div >
	);
}

export default Dashboard;
