import "./Dashboard.css";
import Button from "./Button";
import MessageContainer from "./messageContainer";

function Dashboard(props) {
	return (
		<div className="Dashboard">
			<h1 className="Dashboard-header">{props.name || "Dashboard"}</h1>
			<img className="Dashboard-logo" src={props.logo || ""} alt="logo" />
			<div className="Dashboard-buttons">
				<Button text="Send" />
				<Button text="Receive" />
			</div>
			<div className="Message-container">
			<MessageContainer />
			</div>
		</div >
	);
}

export default Dashboard;
