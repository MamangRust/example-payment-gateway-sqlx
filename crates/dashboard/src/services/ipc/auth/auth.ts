import { LoginRequest, RegisterRequest } from "@/types/domain/request";
import {
  ApiResponseLogin,
  ApiResponseGetMe,
  ApiResponseRegister,
  ApiResponseRefreshToken,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class AuthCommand {
  async login(req: LoginRequest): Promise<ApiResponseLogin> {
    try {
      const response = await invoke<ApiResponseLogin>("login", { req });
      return response;
    } catch (error) {
      throw new Error("Login failed");
    }
  }

  async register(req: RegisterRequest): Promise<ApiResponseRegister> {
    try {
      const response = await invoke<ApiResponseRegister>("register", { req });
      return response;
    } catch (error) {
      throw new Error("Register failed");
    }
  }

  async getMe(accessToken: string): Promise<ApiResponseGetMe> {
    try {
      const response = await invoke<ApiResponseGetMe>("get_me", {
        access_token: accessToken,
      });
      return response;
    } catch (error) {
      throw new Error("Get me failed");
    }
  }

  async refreshToken(
    accessToken: string,
    refreshToken: string,
  ): Promise<ApiResponseRefreshToken> {
    try {
      const response = await invoke<ApiResponseRefreshToken>(
        "myrefresh_token",
        {
          access_token: accessToken,
          refresh_token: refreshToken,
        },
      );
      return response;
    } catch (error) {
      throw new Error("Refresh token failed");
    }
  }
}

export default new AuthCommand();
