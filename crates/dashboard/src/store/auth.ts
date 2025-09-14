import { create } from "zustand";
import { persist } from "zustand/middleware";
import { handleApiError } from "@/helpers/handleApi";
import { AuthStore } from "@/types/state/auth";
import AuthService from "@/services/api/auth/auth";
import AuthCommand from "@/services/api/auth/auth";
import { isTauri } from "@tauri-apps/api/core";
import { LoginRequest, RegisterRequest } from "@/types/domain/request";

const useAuthStore = create<AuthStore>()(
  persist<AuthStore>(
    (set, get) => ({
      user: null,
      accessToken: null,
      refreshToken: null,
      isAuthenticated: false,

      refreshTimer: null,

      loadingLogin: false,
      loadingRegister: false,
      loadingLogout: false,
      loadingGetMe: false,
      loadingRefreshAccessToken: false,

      errorLogin: null,
      errorRegister: null,
      errorLogout: null,
      errorGetMe: null,
      errorRefreshAccessToken: null,

      setLoadingLogin: (value: boolean) => set({ loadingLogin: value }),
      setLoadingRegister: (value: boolean) => set({ loadingRegister: value }),
      setLoadingLogout: (value: boolean) => set({ loadingLogout: value }),
      setLoadingGetMe: (value: boolean) => set({ loadingGetMe: value }),
      setLoadingRefreshAccessToken: (value: boolean) =>
        set({ loadingRefreshAccessToken: value }),

      setErrorLogin: (value: string | null) => set({ errorLogin: value }),
      setErrorRegister: (value: string | null) => set({ errorRegister: value }),
      setErrorLogout: (value: string | null) => set({ errorLogout: value }),
      setErrorGetMe: (value: string | null) => set({ errorGetMe: value }),
      setErrorRefreshAccessToken: (value: string | null) =>
        set({ errorRefreshAccessToken: value }),

      login: async (req: LoginRequest, toast: any) => {
        set({
          loadingLogin: true,
          errorLogin: null,
          isAuthenticated: false,
        });

        try {

          if (isTauri()) {
            const response = await AuthCommand.login(req);

            set({
              accessToken: response?.access_token,
              refreshToken: response?.refresh_token,
              isAuthenticated: true,
            });

            const timer = setInterval(
              () => get().refreshAccessToken?.(toast),
              15 * 60 * 1000,
            );

            set({
              refreshTimer: timer,
            });
          } else {
            const response = await AuthService.login(req);

            console.log("auth store",response)

            set({
              accessToken: response?.access_token,
              refreshToken: response?.refresh_token,
              isAuthenticated: true,
            });

            const timer = setInterval(
              () => get().refreshAccessToken?.(toast),
              15 * 60 * 1000,
            );

            set({
              refreshTimer: timer,
            });
          }



          return true;
        } catch (error) {
          handleApiError(
            error,
            () =>
              set({
                loadingLogin: false,
              }),
            (message: any) =>
              set({
                errorLogin: message,
              }),
            toast,
          );
          return false;
        }
      },
      logout: async (toast: any) => {
        set({
          loadingLogout: true,
          errorLogout: null,
        });

        try {
          set({
            user: null,
            accessToken: null,
            refreshToken: null,
          });
          return true;
        } catch (error: any) {
          handleApiError(
            error,
            () =>
              set({
                loadingLogout: false,
              }),
            (message: any) =>
              set({
                errorLogout: message,
              }),
            toast,
          );
          return false;
        }
      },

      register: async (req: RegisterRequest, toast: any) => {
        set({
          loadingRegister: true,
          errorRegister: null,
        });
        try {
          if (isTauri()) {
            await AuthCommand.register(req);

            set({
              loadingRegister: false,
              errorRegister: null,
            });

            return true;
          } else {
            await AuthService.register(req);


            set({
              loadingRegister: false,
              errorRegister: null,
            });


            return true;
          }





        } catch (error) {
          handleApiError(
            error,
            () =>
              set({
                loadingRegister: false,
              }),
            (message: any) =>
              set({
                errorRegister: message,
              }),
            toast,
          );
          return false;
        }
      },

      getMe: async (toast: any) => {
        set({
          loadingGetMe: true,
          errorGetMe: null,
        });

        try {
          const accessToken = get().accessToken;
          if (!accessToken) {
            throw new Error("No access token found.");
          }

          if (isTauri()) {
            const response = await AuthCommand.getMe(accessToken)

            set({
              loadingGetMe: false,
              errorGetMe: null,
              user: response,
            });

          } else {
            const response = await AuthService.getMe(accessToken);

            set({
              loadingGetMe: false,
              errorGetMe: null,
              user: response,
            });
          }

        } catch (error) {
          handleApiError(
            error,
            () =>
              set({
                loadingGetMe: false,
              }),
            (message: any) =>
              set({
                errorGetMe: message,
              }),
            toast,
          );
        }
      },

      refreshAccessToken: async (toast: any) => {
        set({
          loadingRefreshAccessToken: true,
          errorRefreshAccessToken: null,
        });
        try {
          const accessToken = get().accessToken;
          if (!accessToken) {
            throw new Error("No access token found.");
          }

          const refreshToken = get().accessToken;
          if (!refreshToken) {
            throw new Error("No refresh token found.");
          }

          if (isTauri()) {

            const response = await AuthCommand.refreshAccessToken(accessToken, refreshToken);
            set({
              loadingRefreshAccessToken: false,
              errorRefreshAccessToken: null,
              refreshToken: response?.refresh_token,
              accessToken: response?.access_token,
            });

          } else {
            const response = await AuthService.refreshAccessToken(
              accessToken,
              refreshToken,
            );
            set({
              loadingRefreshAccessToken: false,
              errorRefreshAccessToken: null,
              refreshToken: response?.refresh_token,
              accessToken: response?.access_token,
            });
          }


        } catch (error) {
          handleApiError(
            error,
            () =>
              set({
                loadingRefreshAccessToken: false,
              }),
            (messsage: any) =>
              set({
                errorRefreshAccessToken: messsage,
              }),
            toast,
          );
        }
      },
    }),
    {
      name: "auth",
    },
  ),
);

export const getAccessToken = () => {
  const { accessToken } = useAuthStore.getState();

  console.log("access token", accessToken);

  if (!accessToken) {
    throw new Error("Access token not found");
  }

  return accessToken;
};

export const handleUnauthorized = (error: any, toast: any) => {
  if (error) {
    const { logout } = useAuthStore.getState();

    const authStorage = localStorage.getItem("auth-storage");

    if (authStorage) {
      logout(toast);
    }
  }
};

export default useAuthStore;
