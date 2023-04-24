import React, { useState } from "react";
import cx from "classnames";
import { messages as mockMessages, user1 as ownerUser, user2 } from "./Chat";
import styles from "../chat/Chat.module.css";

function Chat() {
  const [messages, setMessages] = useState(mockMessages);
  const [input, setInput] = useState("");

  const handleSubmit = (event) => {
    event.preventDefault();
    setMessages([...messages, { text: input, sender: "user" }]);
    setInput("");
  };

  return (
    <div className="chat">
      <div className="chat-history">
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
              className={cx(styles.message, {
                [styles.message_owner]: isOwnerMessage,
              })}
            >
              <div className={styles.nameUser}>{name}</div>
              {message.text}
            </div>
          );
        })}
      </div>
      <form className={styles.writeText} onSubmit={handleSubmit}>
        <input
          className={styles.text}
          type="text"
          placeholder="Type your message..."
          value={input}
          onChange={(event) => setInput(event.target.value)}
        />
        <button type="submit">Send</button>
      </form>
    </div>
  );
}

// type Props = {
//   data: {
//     firstName: string;
//     lastName: string;
//     age: number;
//   }[];
// };

// export function TestComponent(props: Props) {
//   const { data } = props;
//   return (
//     <>
//       {data.map((userData, index) => {
//         const { firstName, lastName, age } = userData;
//         return (
//           <div key={index}>
//             <div>Имя: {firstName}</div>
//             <div>Фамилия: {lastName}</div>
//             <div>Возраст: {age}</div>
//           </div>
//         );
//       })}
//     </>
//   );
// }

export default Chat;
