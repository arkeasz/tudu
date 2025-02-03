import { NavLink } from "react-router";
import { useNavigate } from "react-router";
import './Home.css'
import { useEffect } from "react";

function Home() {
    const navigate = useNavigate();

    useEffect(() => {
        const isLogin = localStorage.getItem("login") == 'yes';
        if (isLogin) {
            navigate('/dashboard')
        }
    }, [navigate])

    return (
        <>
            <h1 className="logo-title">TuDu</h1>
            <p className="slogan">Organize, Collaborate, Achieve.</p>
            <div className="links">
                <NavLink className="login" to="/login"> Sign in </NavLink>
                <NavLink className="register" to="/register"> Sign up </NavLink>
            </div>
        </>
    );
}

export default Home;
