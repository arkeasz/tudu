import { useEffect } from "react";
import { useNavigate } from "react-router";
import Sidebar from '../components/SideBar'
import MainContent from '../components/MainContent'
import Navbar from '../components/NavBar'
import './Dashboard.css'

function Dashboard() {
  const navigate = useNavigate();

  useEffect(() => {
    if (localStorage.getItem("login") !== "yes") {
      navigate("/login");
    }
  }, [navigate]);

  return (
    <div className="tudu__dashboard">
        <Sidebar />
        <div>
            <Navbar />
            <MainContent />
        </div>
    </div>
  );
}

export default Dashboard;
