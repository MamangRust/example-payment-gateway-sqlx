import { useEffect } from "react";
import axios from "axios";
import { jwtDecode } from "jwt-decode";
import useAuthStore from "@/store/auth";
import { useToast } from "../use-toast";

export const useAxiosInterceptor = () => {
  const { accessToken, logout } = useAuthStore();
  const { toast } = useToast();

  useEffect(() => {
    const requestInterceptor = axios.interceptors.request.use(
      (config) => {
        if (accessToken) {
          const decodedToken: { exp: number } = jwtDecode(accessToken);
          const currentTime = Date.now() / 1000;

          if (decodedToken.exp < currentTime) {
            logout(toast);
          } else {
            config.headers.Authorization = `Bearer ${accessToken}`;
          }
        }
        return config;
      },
      (error) => {
        return Promise.reject(error);
      },
    );

    return () => {
      axios.interceptors.request.eject(requestInterceptor);
    };
  }, [accessToken, logout]);
};
