import React, { useState } from "react";
import cx from "classnames";
import {
  messages as mockMessages,
  user1 as ownerUser,
  user2,
} from "./mockMessages";
import styles from "../chat/Chat.module.css";

function Chat() {
  let today = new Date();
  let now = today.toLocaleTimeString("en-US");
  const [messages, setMessages] = useState(mockMessages);
  const [input, setInput] = useState("");

  const handleSubmit = (event) => {
    event.preventDefault();
    setMessages([...messages, { text: input, sender: "user" }]);
    setInput("");
  };

  return (
    <div>
      {/* Bubble message */}
      <div>
        {messages.map((message) => {
          const isOwnerMessage = message.senderId === ownerUser.id;
          let name;
          if (isOwnerMessage) {
            name = ownerUser.nickname;
          } else {
            name = user2.nickname;
          }
          return (
            <div
              key={message.id}
              className={cx(styles.messageBubble, {
                [styles.messageBubble_owner]: isOwnerMessage,
              })}
            >
              {/* <div className={styles.messageTime}>{now}</div> */}
              <div className={styles.messageText}>{message.text}</div>
            </div>
          );
        })}
      </div>
      {/* Input Window */}
      <div className={styles.containerForInput}>
        <form className={styles.writeText} onSubmit={handleSubmit}>
          <input
            className={styles.inputField}
            type="text"
            placeholder="Type your message..."
            value={input}
            onChange={(event) => setInput(event.target.value)}
          />
          <button className={styles.sendButton} type="submit">
            Send
          </button>
        </form>
      </div>
    </div>
  );
}

export default Chat;
