import myApi from "@/helpers/api";
import { DeletePermanentUser } from "@/types/domain/request";
import { FindAllUserTrashed, RestoreUserTrashed } from "@/types/domain/request";
import {
  ApiResponsePaginationUserDeleteAt,
  ApiResponseUser,
  ApiResponseUserAll,
  ApiResponseUserDelete,
} from "@/types/domain/response";

class UserTrashedService {
  async findAllUsersTrashed(
    req: FindAllUserTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationUserDeleteAt> {
    try {
      const response = await myApi.get("/user/trashed", {
        params: {
          page: req.page,
          page_size: req.page_size,
          search: req.search,
        },
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data;
      }

      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async restoreUserTrashed(
    req: RestoreUserTrashed,
    access_token: string,
  ): Promise<ApiResponseUser["data"]> {
    try {
      const response = await myApi.post(`/user/restore/${req.id}`, {
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data.data;
      }

      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async deletePermanentUser(
    req: DeletePermanentUser,
    access_token: string,
  ): Promise<ApiResponseUserDelete> {
    try {
      const response = await myApi.delete(`/user/delete/${req.id}`, {
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data.data;
      }

      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async restoreUserAllTrashed(
    access_token: string,
  ): Promise<ApiResponseUserAll> {
    try {
      const response = await myApi.post(`/user/restore-all`, null, {
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data.data;
      }

      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async deletePermanentAllUser(
    access_token: string,
  ): Promise<ApiResponseUserAll> {
    try {
      const response = await myApi.post(`/user/delete-all`, null, {
        headers: { Authorization: `Bearer ${access_token}` },
      });

      if (response.status == 200) {
        return response.data.data;
      }

      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
}

export default new UserTrashedService();
