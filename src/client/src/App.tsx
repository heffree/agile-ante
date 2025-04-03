import { Routes, Route, BrowserRouter } from 'react-router-dom';
import './App.css';
import { Home, Room } from './views/';

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/room/:id" element={<Room />} />
      </Routes>
    </BrowserRouter>
  );

}

