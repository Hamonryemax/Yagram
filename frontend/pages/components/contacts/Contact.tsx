import React from "react";
import Image, { StaticImageData } from "next/image";
import styles from "./Contact.module.css";
import { style } from "@mui/system";
import ContactElement from "./ContactElement";

type Props = {
  contacts: {
    id: string;
    name: string;
    lastMessage: string;
    avatar: StaticImageData;
    isOnline: boolean;
  }[];
};

function ContactList(props: Props) {
  const contacts = props.contacts;

  return (
    <ul className={styles.ul}>
      {contacts.map((contact) => (
        <ContactElement
          name={contact.name}
          lastMessage={contact.lastMessage}
          avatar={contact.avatar}
          isOnline={contact.isOnline}
          key={contact.id}
        />
      ))}
    </ul>
  );
}

export default ContactList;
