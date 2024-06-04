import { SignState } from "../../models/SignState";
import { Context } from "../../main";
import { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

export default function Phone() {
  const [phone, setPhone] = useState("+");
  const { store } = useContext(Context);
  const navigate = useNavigate();

  useEffect(() => {
    store.setSignState(SignState.Qr);
  });

  async function sendCode() {
    console.log(phone)
    await invoke("request_code", { phone: phone });
    navigate('/code')
  }

  return (
    <div className="h-screen w-screen flex flex-col gap-2 justify-center items-center bg-black text-white">
      <label htmlFor="phone">Phone</label>
      <input
        id="phone"
        placeholder="Phone"
        className="bg-inherit rounded-lg p-2 outline-none border-2 border-gray-800 text-gray-500 placeholder-slate-800"
        value={phone}
        onChange={(e) => {
          const value = e.target.value;
          if (!value.startsWith("+")) return;
          if (value.length > 1 && isNaN(value as any)) return;

          setPhone(value);
        }}
      />
      <button
        onClick={sendCode}
        className="p-2 bg-gray-800 rounded-lg hover:opacity-80 transition-all duration-200 ms border-2 border-gray-800"
      >
        Continue
      </button>
      <p>
        or continue with{" "}
        <a href="/" className="underline underline-offset-2 text-gray-500">
          QR-Code
        </a>
      </p>
    </div>
  );
}
