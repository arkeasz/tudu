import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [data, setData] = useState([]);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const getPosts = async () =>  {
      const uri = import.meta.env.VITE_API_URL
      try {
        const res = await fetch(`${uri}posts`);
        if (!res.ok)  {
          throw new Error(`response status: ${res.status}`);
        }
        const result = await res.json();
        console.log(result);
        setData(result)
      } catch (error) {
        console.error(error.message);
        setError(error)
      } finally {
        setLoading(false)
      }
    };
    getPosts()
  }, [])

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <>
      <div>
        ---
      </div>
      {
        data.map((e, index) => {
          return (
            <div key={index}>
              <p>{e.title}</p>
              <p>{e.body}</p>
            </div>
          )
        })
      }
      <div>
        ---
      </div>
    </>
  );
}

export default App;
