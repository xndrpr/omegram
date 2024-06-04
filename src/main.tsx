import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import './globals.css';
import {createBrowserRouter, RouterProvider} from 'react-router-dom';
import Phone from "./pages/Phone/Phone";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App/>
  },
  {
    path: '/phone',
    element: <Phone />
  }
]);
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router}/>
  </React.StrictMode>,
);
