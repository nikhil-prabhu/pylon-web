function MessageContainer() {
    return(
        <div>
            <label>Send Message:</label>
            <input type="text" name="sendMsg"></input><label></label>
            <label>Received Message:</label>
            <input type="text" name="receiveMsg"></input>
        </div>
    );
}

export default MessageContainer;