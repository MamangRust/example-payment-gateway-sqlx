import axios from "axios";

const myApi = axios.create({
  baseURL:
    import.meta.env.VITE_BASE_URL + "/api" || "http://localhost:5000/api",
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
  withCredentials: true,
});

export default myApi;
