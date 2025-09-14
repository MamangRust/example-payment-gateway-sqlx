import myApi from "@/helpers/api";
import { LoginRequest, RegisterRequest } from "@/types/domain/request";
import {
  ApiResponseLogin,
  ApiResponseRegister,
  ApiResponseGetMe,
  ApiResponseRefreshToken,
} from "@/types/domain/response";

class AuthService {
  /**
   * Login user ke dalam aplikasi
   * @param req LoginRequest
   * @returns TokenResponse (access_token & refresh_token)
   */
  async login(req: LoginRequest): Promise<ApiResponseLogin["data"]> {
    try {
      const response = await myApi.post<ApiResponseLogin>("/auth/login", req);

      console.log("hello", response)
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  /**
   * Register user baru
   * @param req RegisterRequest
   * @returns UserResponse
   */
  async register(req: RegisterRequest): Promise<ApiResponseRegister["data"]> {
    try {
      const response = await myApi.post<ApiResponseRegister>(
        "/auth/register",
        req,
      );
      if (response.status === 201) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Registration failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Registration failed.");
    }
  }

  /**
   * Mendapatkan data user saat ini
   * @param accessToken string
   * @returns UserResponse
   */
  async getMe(accessToken: string): Promise<ApiResponseGetMe["data"]> {
    try {
      const response = await myApi.get<ApiResponseGetMe>("/auth/me", {
        headers: {
          Authorization: `Bearer ${accessToken}`,
        },
      });
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Failed to fetch user data.");
    } catch (error: any) {
      throw new Error(
        error.response?.data?.message || "Failed to fetch user data.",
      );
    }
  }

  /**
   * Memperbarui access token menggunakan refresh token
   * @param refreshToken string
   * @returns TokenResponse (access_token & refresh_token)
   */
  async refreshAccessToken(
    accessToken: string,
    refreshToken: string,
  ): Promise<ApiResponseRefreshToken["data"]> {
    try {
      const response = await myApi.post<ApiResponseRefreshToken>(
        "/auth/refresh-token",
        {
          refresh_token: refreshToken,
        },
        {
          headers: {
            Authorization: `Bearer ${accessToken}`,
          },
        },
      );
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(
        response.data.message || "Failed to refresh access token.",
      );
    } catch (error: any) {
      throw new Error(
        error.response?.data?.message || "Failed to refresh access token.",
      );
    }
  }
}

export default new AuthService();
