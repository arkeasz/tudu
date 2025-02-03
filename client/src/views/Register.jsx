import { useState } from "react";
import { NavLink } from "react-router";
import Header from "../components/Header";

function Register() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [username, setUsername] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");

    if (!email || !password || !username) {
      setError("Todos los campos son obligatorios.");
      return;
    }

    try {
      const response = await fetch("http://localhost:8080/users/register", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password, username }),
      });

      if (!response.ok) throw new Error("Error en el registro");

      const data = await response.json();
      console.log("Usuario registrado:", data);
    } catch (err) {
      setError(err.message);
    }
  };

  return (
    <>
      <Header />
      <div className="tudu_signup">
        {error && <p>{error}</p>}

        <form onSubmit={handleSubmit}>
          <input type="text" placeholder="Username" value={username} onChange={(e) => setUsername(e.target.value)} />
          <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
          <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
          <button type="submit">Sign Up</button>
        </form>

        <p>
          Do you have an account?
        </p>
        <NavLink to="/login">Sign in</NavLink>
      </div>
    </>
  );
}

export default Register;
