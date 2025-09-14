import { User } from "@/types/model";

export interface TokenResponse {
  access_token: string;
  refresh_token: string;
}

export interface ApiResponseLogin {
  status: string;
  message: string;
  data?: TokenResponse;
}

export interface ApiResponseRegister {
  status: string;
  message: string;
  data?: User;
}

export interface ApiResponseRefreshToken {
  status: string;
  message: string;
  data?: TokenResponse;
}

export interface ApiResponseGetMe {
  status: string;
  message: string;
  data?: User;
}
