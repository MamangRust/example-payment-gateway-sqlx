import myApi from "@/helpers/api";
import {
  DeletePermanentTopup,
  FindAllTopupTrashed,
  RestoreTopupTrashed,
} from "@/types/domain/request/topup";
import {
  ApiResponsePaginationTopupDeleteAt,
  ApiResponseTopup,
  ApiResponseTopupAll,
  ApiResponseTopupDelete,
} from "@/types/domain/response";

class TopupTrashedService {
  async findAllTopupsTrashed(
    req: FindAllTopupTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationTopupDeleteAt> {
    try {
      const response = await myApi.get("/topups/trashed", {
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
  async restoreTopupTrashed(
    req: RestoreTopupTrashed,
    access_token: string,
  ): Promise<ApiResponseTopup["data"]> {
    try {
      const response = await myApi.post(`/topups/restore/${req.id}`, {
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
  async deletePermanentTopup(
    req: DeletePermanentTopup,
    access_token: string,
  ): Promise<ApiResponseTopupDelete> {
    try {
      const response = await myApi.delete(`/topups/delete/${req.id}`, {
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
  async restoreTopupAllTrashed(
    access_token: string,
  ): Promise<ApiResponseTopupAll> {
    try {
      const response = await myApi.post(`/topups/restore-all`, null, {
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
  async deletePermanentAllTopup(
    access_token: string,
  ): Promise<ApiResponseTopupAll> {
    try {
      const response = await myApi.post(`/topups/delete-all`, null, {
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

export default new TopupTrashedService();
