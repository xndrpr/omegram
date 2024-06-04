import React, { createContext } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./globals.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Phone from "./pages/Phone/Phone";
import Store from "./utils/store";
import Qr from "./pages/Qr/Qr";
import Code from "./pages/Code/Code";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
  },
  {
    path: "/qr",
    element: <Qr />,
  },
  {
    path: "/code",
    element: <Code />,
  },
  {
    path: "/phone",
    element: <Phone />,
  },
]);

interface State {
  store: Store;
}
const store = new Store();
export const Context = createContext<State>({ store });
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <Context.Provider value={{ store }}>
    <RouterProvider router={router} />
  </Context.Provider>
);
