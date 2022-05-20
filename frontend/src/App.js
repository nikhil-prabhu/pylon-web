import './App.css';
import logo from "./assets/logo.gif";

import Dashboard from "./components/Dashboard";

function App() {
  return (
    <div className="App">
      <Dashboard name="Pylon" logo={logo} />
    </div>
  );
}

export default App;
