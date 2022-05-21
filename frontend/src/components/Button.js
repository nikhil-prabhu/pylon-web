import "./Button.css";

function Button(props) {
	return (
		<button className="Button" type="button" onClick={props.onClick}>{props.text || "Button"}</button>
	);
}

export default Button;
