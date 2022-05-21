import "./Dashboard.css";
import MessageContainer from "./MessageContainer";

function Dashboard(props) {
	return (
		<div className="Dashboard">
			<h1 className="Dashboard-header">{props.name || "Dashboard"}</h1>
			<img className="Dashboard-logo" src={props.logo || ""} alt="logo" />
			
			<div className="Message-container">
			<MessageContainer />
			</div>
		</div >
	);
}

export default Dashboard;
