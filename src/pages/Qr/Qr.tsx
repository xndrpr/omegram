import { SignState } from "../../models/SignState";
import { Context } from "../../main";
import { invoke } from "@tauri-apps/api/tauri";
import { QRCodeSVG } from "qrcode.react";
import { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Qr() {
  const [qr, setQr] = useState<string>("");
  const { store } = useContext(Context);
  const navigate = useNavigate();

  useEffect(() => {
    async function getQr() {
      setQr(await invoke("request_qrcode"));
    }

    getQr();
  }, []);

  useEffect(() => {
    store.setSignState(SignState.Qr);
  })

  async function signIn() {
    const result = await invoke("get_update");
    if (result) {
      store.setSignState(SignState.None);
      await invoke("set_setting", { key: "auth", value: "true" });
      navigate('/');
    }
  }

  async function refresh() {
    setQr(await invoke("request_qrcode"));
  }

  return (
    <div className="dark h-screen w-screen flex flex-col justify-center items-center bg-black text-white">
      <QRCodeSVG value={qr} className="h-1/3 w-auto border-4 rounded-xl" />
      <button
        onClick={signIn}
        className="p-2 bg-gray-800 rounded-lg hover:opacity-80 transition-all duration-200 ms border-2 border-gray-800"
      >
        Continue
      </button>
      <button
        onClick={refresh}
        className="p-2 bg-gray-800 rounded-lg hover:opacity-80 transition-all duration-200 ms border-2 border-gray-800"
      >
        Refresh
      </button>
      <p>
        Or continue with{" "}
        <a href="/phone" className="underline underline-offset-2 text-gray-500">
          phone
        </a>
      </p>
    </div>
  );
}
