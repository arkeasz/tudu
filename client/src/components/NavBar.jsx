import { useNavigate } from "react-router";

function Navbar() {
    const navigate = useNavigate();

    const handleLogout = () => {
      localStorage.removeItem("login");
      navigate("/");
    };

    return (
      <nav className="tudu__dashboard-nav">
        <h1>Dashboard</h1>
        <div>
          <input type="text" placeholder="Buscar..." />
          <button>Perfil</button>
          <button onClick={handleLogout}>Cerrar sesión</button>
        </div>
      </nav>
    );
}

export default Navbar;
