import { useState, useEffect } from "react";
import { useNavigate, NavLink } from "react-router";
import Header from "../components/Header";
import '../assets/Form.css'

function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();

  useEffect(() => {
    if (localStorage.getItem("login") === "yes") {
      navigate("/dashboard");
    }
  }, [navigate]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");

    if (!email || !password) {
      setError("All fields are required.");
      return;
    }

    try {
      const response = await fetch("http://localhost:8080/users/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
      });

      if (response.ok) {
        localStorage.setItem("login", "yes");
        navigate("/dashboard");
      } else {
        setError("User not found or incorrect credentials.");
      }
    } catch (err) {
      setError("Server connection error.");
    }
  };

  return (
    <>
      <Header />
      <div className="tudu_signin">
        {error && <p>{error}</p>}

        <form onSubmit={handleSubmit}>
          <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
          <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
          <button className="signin" type="submit">Sign In</button>
        </form>

        <p>
          Dont have an account?
        </p>
        <NavLink to="/register">Sign Up</NavLink>
      </div>
    </>
  );
}

export default Login;
