import { LoginRequest, RegisterRequest } from "../domain/request";
import { User } from "../model/user";

export interface AuthStore {
  user: User | null;
  accessToken: string | null;
  refreshToken: string | null;
  refreshTimer: any;
  isAuthenticated: boolean;

  loadingLogin: boolean;
  loadingRegister: boolean;
  loadingLogout: boolean;
  loadingGetMe: boolean;
  loadingRefreshAccessToken: boolean;

  errorLogin: string | null;
  errorRegister: string | null;
  errorLogout: string | null;
  errorGetMe: string | null;
  errorRefreshAccessToken: string | null;

  setErrorLogin: (value: string | null) => void;
  setErrorRegister: (value: string | null) => void;
  setErrorLogout: (value: string | null) => void;
  setErrorGetMe: (value: string | null) => void;
  setErrorRefreshAccessToken: (value: string | null) => void;

  setLoadingLogin: (value: boolean) => void;
  setLoadingRegister: (value: boolean) => void;
  setLoadingLogout: (value: boolean) => void;
  setLoadingGetMe: (value: boolean) => void;
  setLoadingRefreshAccessToken: (value: boolean) => void;

  login: (req: LoginRequest, toast: any) => Promise<boolean>;
  register: (req: RegisterRequest, toast: any) => Promise<boolean>;
  getMe: (toast: any) => Promise<void>;
  refreshAccessToken: (toast: any) => Promise<void>;
  logout: (toast: any) => Promise<boolean>;
}
