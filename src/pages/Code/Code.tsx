import { SignState } from "../../models/SignState";
import { Context } from "../../main";
import { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

export default function Code() {
  const [code, setCode] = useState("+");
  const { store } = useContext(Context);
  const navigate = useNavigate();

  useEffect(() => {
    store.setSignState(SignState.Code);
  });

  async function signIn() {
    navigate("/");
  }

  return (
    <div className="h-screen w-screen flex flex-col gap-2 justify-center items-center bg-black text-white">
      <label htmlFor="phone">Code</label>

      <button
        onClick={signIn}
        className="p-2 bg-gray-800 rounded-lg hover:opacity-80 transition-all duration-200 ms border-2 border-gray-800"
      >
        Continue
      </button>
      <p>
        or go{" "}
        <a href="/phone" className="underline underline-offset-2 text-gray-500">
          back
        </a>
      </p>
    </div>
  );
}
