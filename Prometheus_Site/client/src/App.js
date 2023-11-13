import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Docs from "./components/Docs";
import Download from "./components/Download";
import Home from "./components/Home";

const App = () => {
  return (
    <div>
      <BrowserRouter>
        <Routes>
          <Route path='/' element={<Home />} />
          <Route path='/documentation' element={<Docs />} />
          <Route path='/download' element={<Download />} />
        </Routes>
      </BrowserRouter>
    </div>
  )
}

export default App;
