import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from 'react-router'
import Home from "./views/Home";
import Login from "./views/Login";
import Register from "./views/Register";
import Dashboard from "./views/Dashboard";
import './main.css'

const root = document.getElementById("root")

ReactDOM.createRoot(root).render(
  <BrowserRouter>
    <Routes>
      <Route  path="/" element={<Home />} />
      <Route  path="/login" element={<Login />} />
      <Route  path="/register" element={<Register />} />
      <Route  path="/dashboard" element={<Dashboard />} />
    </Routes>
  </BrowserRouter>,
);
