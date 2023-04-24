import React from "react";
import Image, { StaticImageData } from "next/image";
import styles from "./Contact.module.css";
import { style } from "@mui/system";

type Props = {
  name: string;
  lastMessage: string;
  avatar: StaticImageData;
  isOnline: boolean;
};

function ContactElement(props: Props) {
  const { name, lastMessage, avatar, isOnline } = props;

  return (
    <li className={styles.contact}>
      <Image className={styles.image} src={avatar} alt={name} />
      <div>
        <div className={styles.wrapperNameStatus}>
          <div className={styles.name}>{name}</div>
          {isOnline ? (
            <div className={styles.statusOnline}></div>
          ) : (
            <div className={styles.statusOffline}></div>
          )}
        </div>
        <div className={styles.lastMessage}>{lastMessage}</div>
      </div>
    </li>
  );
}

export default ContactElement;
