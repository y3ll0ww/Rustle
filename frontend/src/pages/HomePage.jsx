import { Link } from "react-router-dom";

function App() {
  return (
    <div className="p-6">
      <h1 className="text-3xl font-bold">Welcome to Rustle</h1>
      <p className="mt-4">This is your main landing page.</p>
      <Link
        to="/login"
        className="inline-block mt-6 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
      >
        Go to Login
      </Link>
    </div>
  );
}

export default App;
