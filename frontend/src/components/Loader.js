import "./Loader.css";

function Loader(props) {
	return (
		<div className="Loader">
			<div className="Loader-spinner"></div>
			{props.text || "Loading"}
		</div>
	)
}

export default Loader;
