import { SignState } from "@/models/SignState";
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

  return (
    <div className="dark h-screen w-screen flex flex-col justify-center items-center bg-black text-white">
      <QRCodeSVG value={qr} className="h-1/3 w-auto border-4 rounded-xl" />
      <p>
        Or continue with{" "}
        <a href="/phone" className="underline underline-offset-2 text-gray-500">
          phone
        </a>
      </p>
    </div>
  );
}
