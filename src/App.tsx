import { invoke } from "@tauri-apps/api/tauri";
import { useContext, useEffect, useState } from "react";

import { Context } from "./main";
import { useNavigate } from "react-router-dom";
import { observer } from "mobx-react-lite";

function App() {
  const { store } = useContext(Context);
  const navigate = useNavigate();
  const [dialogs, setDialogs] = useState<any>();

  useEffect(() => {
    async function initialize() {
      const result = await invoke("check_auth");

      if (!result) {
        navigate("/qr");
      } else {
        store.setAuth(true);

        let needed = await invoke("update_dialogs");
        if (!needed) return;

        let dlgs = JSON.parse(localStorage.getItem("dialogs") || "");
        setDialogs(dlgs);
        await invoke("set_setting", {
          key: "dialogs",
          value: JSON.stringify(dlgs),
        });
      }
    }

    initialize();
  });

  function bytesToPhoto(bytes: any) {
    return `data:image/png;base64,${btoa(String.fromCharCode(...bytes))}`;
  }

  return (
    <div className="dark h-screen w-screen grid grid-cols-1 justify-center items-center bg-black text-white">
      <div className="dark col-end-1 h-full max-h-screen overflow-auto w-64">
        {dialogs && dialogs.map((dialog: any) => (
          <button className="w-full flex justify-start gap-2 items-center border-b-2 border-r-2 border-gray-800 rounded-md p-3">
            <img
                width="50px"
                height="50px"
                src={bytesToPhoto(dialog.photo)}
                className=" rounded-full"
                alt="avatar"
              />
              <p>{dialog.name}</p>
          </button>
        ))}
      </div>
      <div className="h-full w-full">
      <p className="flex justify-center items-center h-screen">Select a chat to start messaging</p>
      </div>
    </div>
  );
}

export default observer(App);
