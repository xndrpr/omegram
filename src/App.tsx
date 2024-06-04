import { invoke } from "@tauri-apps/api/tauri";
import { useContext, useEffect, useState } from "react";

import { Context } from "./main";
import { useNavigate } from "react-router-dom";
import {observer} from "mobx-react-lite";

function App() {
  const { store } = useContext(Context);
  const navigate = useNavigate();


  useEffect(() => {
    async function checkAuth() {
      const result = await invoke("check_auth");

      if (!result) {
        navigate("/qr");
      } else {
        store.setAuth(true);
      }
    }

    checkAuth();
  })

  return (
    <div className="dark h-screen w-screen flex flex-col justify-center items-center bg-black text-white">
      <h1>Hi</h1>
    </div>
  );
}

export default observer(App);
