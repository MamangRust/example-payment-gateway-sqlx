import { useEffect, useState } from "react";
import { Navigate, useLocation } from "react-router-dom";
import useAuthStore from "../store/auth";
import { jwtDecode } from "jwt-decode";
import usePreviousPath from "@/hooks/utils/usePreviousPath";
import { useToast } from "@/hooks/use-toast";

const AuthProvider = ({ children }: any) => {
  const {
    refreshAccessToken,
    isAuthenticated,
    logout,
    accessToken,
    user,
    getMe,
  } = useAuthStore();
  const { pathname } = useLocation();
  const previousPath = usePreviousPath();
  const [requestedLocation, setRequestedLocation] = useState<string | null>(
    null,
  );
  const { toast } = useToast();

  useEffect(() => {
    if (!accessToken) {
      toast({
        title: "Error",
        description: "You are not logged in",
        variant: "destructive",
      });
      setRequestedLocation(pathname);
      return;
    }

    if (isAuthenticated && accessToken) {
      try {
        const decodedToken: { exp?: number } = jwtDecode(accessToken);
        const expirationTime = decodedToken.exp ? decodedToken.exp * 1000 : 0;
        const currentTime = Date.now();
        const timeRemaining = expirationTime - currentTime;

        if (timeRemaining <= 0) {
          logout(toast);
          return;
        }

        if (!user) {
          getMe(toast)
            .then(() => {
              console.log("User data fetched successfully");
            })
            .catch((error) => {
              console.error("Failed to fetch user data:", error);
              logout(toast);
            });
        }

        const intervalId = setInterval(
          async () => {
            try {
              console.log("Refreshing token...");
              await refreshAccessToken(toast);
            } catch (error) {
              console.error("Failed to refresh token:", error);
              logout(toast);
            }
          },
          15 * 60 * 1000,
        );

        return () => clearInterval(intervalId);
      } catch (error) {
        console.error("Error decoding token:", error);
        logout(toast);
      }
    }
  }, [
    refreshAccessToken,
    isAuthenticated,
    accessToken,
    logout,
    pathname,
    toast,
    user,
    getMe,
  ]);

  useEffect(() => {
    if (!isAuthenticated && pathname !== requestedLocation) {
      setRequestedLocation(pathname);
    }
  }, [isAuthenticated, pathname, requestedLocation]);

  if (!isAuthenticated) {
    return (
      <Navigate
        to="/auth/login"
        state={{ from: requestedLocation || previousPath }}
      />
    );
  }

  return children;
};

export default AuthProvider;
