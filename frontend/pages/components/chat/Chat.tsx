type User = {
  id: string;
  name: string;
  nickname: string;
};

type Message = {
  id: string;
  senderId: string;
  receiverId: string;
  text: string;
  date: Date;
};

export const user1: User = {
  id: "5",
  name: "Maxim",
  nickname: "Hale Male",
};

export const user2: User = {
  id: "6",
  name: "Kostya",
  nickname: "Slowyn",
};

export const messages: Message[] = [
  {
    id: "0",
    senderId: "5",
    receiverId: "6",
    text: "hello",
    date: new Date(),
  },
  {
    id: "1",
    senderId: "6",
    receiverId: "5",
    text: "hello",
    date: new Date(),
  },
  {
    id: "2",
    senderId: "5",
    receiverId: "6",
    text: "How are you?",
    date: new Date(),
  },
  {
    id: "3",
    senderId: "6",
    receiverId: "5",
    text: "I am fine! Thanks!",
    date: new Date(),
  },
];

function Messages() {
  return messages;
}
