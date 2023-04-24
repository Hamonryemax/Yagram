import styles from "../header/MenuButton.module.css";
import {
  Button,
  Drawer,
  Box,
  Divider,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  Typography,
  useTheme,
} from "@mui/material";
import { useEffect, useState } from "react";
import ContactsOutlinedIcon from "@mui/icons-material/ContactsOutlined";

function MenuButton() {
  const [isOpen, setOpen] = useState(false);

  return (
    <>
      <div className={styles.menu_button} onClick={() => setOpen(true)}>
        <span></span>
      </div>
      <Drawer
        anchor="left"
        open={isOpen}
        onClose={() => {
          setOpen(false);
        }}
      >
        <Box>
          <ContactsOutlinedIcon />
          <Button variant="contained">Contacts</Button>
          <Button variant="contained">Settings</Button>
          <Button variant="contained">About us</Button>
        </Box>
      </Drawer>
    </>
  );
}

export default MenuButton;
