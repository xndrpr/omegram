import { invoke } from "@tauri-apps/api/tauri";
import { useContext, useEffect, useRef, useState } from "react";

import { Context } from "./main";
import { useNavigate } from "react-router-dom";
import { observer } from "mobx-react-lite";

function App() {
  const { store } = useContext(Context);
  const navigate = useNavigate();
  const [dialogs, setDialogs] = useState<any>();
  const [selectedDialog, setSelectedDialog] = useState();
  const [messages, setMessages] = useState([]);
  const [offset, setOffset] = useState(0);
  const limit = 15;

  const endRef = useRef(null);

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

  async function selectChat(dialog: any) {
    setSelectedDialog(dialog);

    const response = await invoke("get_chat_history", { id: dialog.id });

    console.log(response);
    if (response && (response as any)[0].text) {
      setMessages(JSON.parse(response.toString()));
    } else {
      setMessages(
        await invoke("get_messages", {
          id: dialog.id,
          offset: offset,
          limit: limit,
        })
      );
      await invoke("set_chat_history", {
        id: dialog.id,
        history: JSON.stringify(messages),
      });
      setOffset((prev) => prev + limit);
      console.log(messages);
    }
  }

  useEffect(() => {
    if (endRef.current) {
      (endRef.current as any).scrollIntoView();
    }
  }, [messages]);

  return (
    <div className="dark h-screen w-screen grid grid-cols-1 justify-center items-center bg-black text-white">
      <div className="dark col-end-1 h-full max-h-screen overflow-auto w-64">
        {dialogs &&
          dialogs.map((dialog: any) => (
            <button
              key={dialog.id}
              onClick={() => selectChat(dialog)}
              className="w-full flex justify-start gap-2 items-center border-b-2 border-r-2 border-gray-800 rounded-md p-3"
            >
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
        {!selectedDialog || !messages ? (
          <p className="flex justify-center items-center h-screen">
            Select a chat to start messaging
          </p>
        ) : (
          <div className="h-screen flex flex-col">
            <div className="flex-1 overflow-auto">
              {messages &&
                messages.map((message: any) => (
                  <div className="flex justify-start items-end gap-2 p-2 mb-2 mt-2">
                    <img
                      width="36px"
                      height="36px"
                      src={bytesToPhoto(message.avatar)}
                      className="rounded-full"
                      alt="avatar"
                    />
                    <p className="whitespace-pre-wrap">{message.text}</p>
                  </div>
                ))}
              <div ref={endRef}></div>
            </div>
            <div className="flex mt-auto max-w-full">
              <input
                className="w-full p-2 outline-none bg-black border-2 border-gray-800"
                placeholder="Write a message..."
              />
              <button className="bg-gray-800">Send</button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default observer(App);
