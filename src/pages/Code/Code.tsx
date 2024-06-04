import { SignState } from "../../models/SignState";
import { Context } from "../../main";
import { useContext, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

export default function Code() {
  const [code1, setCode1] = useState<string>();
  const [code2, setCode2] = useState<string>();
  const [code3, setCode3] = useState<string>();
  const [code4, setCode4] = useState<string>();
  const [code5, setCode5] = useState<string>();

  const code1ref = useRef<any>();
  const code2ref = useRef<any>();
  const code3ref = useRef<any>();
  const code4ref = useRef<any>();
  const code5ref = useRef<any>();

  const { store } = useContext(Context);
  const navigate = useNavigate();

  useEffect(() => {
    store.setSignState(SignState.Code);
  });

  async function signIn() {
    await invoke("sign_in", {
      code: `${code1}${code2}${code3}${code4}${code5}`,
    });
    await invoke("set_setting", { key: "auth", value: "true" });
    store.setSignState(SignState.None);
    navigate("/");
  }

  return (
    <div className="h-screen w-screen flex flex-col gap-2 justify-center items-center bg-black text-white">
      <label htmlFor="phone">Enter the code you received</label>

      <div className="flex gap-2">
        <input
          ref={code1ref}
          className="w-14 h-14 rounded-md bg-gray-800 text-4xl text-center"
          value={code1}
          onClick={() => setCode1("")}
          onKeyDown={(e) => {
            e.preventDefault();

            if (!isNaN(parseInt(e.key))) {
              setCode1(e.key);
              code2ref.current.focus();
            }
          }}
        />
        <input
          ref={code2ref}
          className="w-14 h-14 rounded-md bg-gray-800 text-4xl text-center"
          value={code2}
          onClick={() => setCode2("")}
          onKeyDown={(e) => {
            e.preventDefault();

            if (e.key === "Backspace") {
              setCode2("");
              setCode1("")
              code1ref.current.focus();
            } else {
              if (!isNaN(parseInt(e.key))) {
                setCode2(e.key);
                code3ref.current.focus();
              }
            }
          }}
        />
        <input
          ref={code3ref}
          className="w-14 h-14 rounded-md bg-gray-800 text-4xl text-center"
          value={code3}
          onClick={() => setCode3("")}
          onKeyDown={(e) => {
            e.preventDefault();

            if (e.key === "Backspace") {
              setCode3("");
              setCode2("")
              code2ref.current.focus();
            } else {
              if (!isNaN(parseInt(e.key))) {
                setCode3(e.key);
                code4ref.current.focus();
              }
            }
          }}
        />
        <input
          ref={code4ref}
          className=" w-14 h-14 rounded-md bg-gray-800 text-4xl text-center"
          value={code4}
          onClick={() => setCode4("")}
          onKeyDown={(e) => {
            e.preventDefault();

            if (e.key === "Backspace") {
              setCode4("");
              setCode3("")
              code3ref.current.focus();
            } else {
              if (!isNaN(parseInt(e.key))) {
                setCode4(e.key);
                code5ref.current.focus();
              }
            }
          }}
        />
        <input
          ref={code5ref}
          className="w-14 h-14 rounded-md bg-gray-800 text-4xl text-center"
          value={code5}
          onClick={() => setCode5("")}
          onKeyDown={async (e) => {
            e.preventDefault();

            if (e.key === "Backspace") {
              setCode5("");
              setCode4("")
              code4ref.current.focus();
            } else {
              if (!isNaN(parseInt(e.key))) {
                setCode5(e.key);

                await signIn();
              }
            }
          }}
        />
      </div>
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
