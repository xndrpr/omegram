import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import {QRCodeSVG} from 'qrcode.react';



function App() {
  const [qr, setQr] = useState<string>("");

  useEffect(() => {
    async function getQr() {
      setQr(await invoke("request_qrcode"));
    }

    getQr();
  }, []);

  return (
    <div className="dark h-screen w-screen flex flex-col justify-center items-center bg-black text-white">
      <QRCodeSVG value={qr} className="h-1/3 w-auto border-4 rounded-xl" />
      <p>Or continue with <a href="/phone" className="underline underline-offset-2 text-gray-500">phone</a></p>
    </div>
  );
}

export default App;
