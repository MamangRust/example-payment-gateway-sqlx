import myApi from "@/helpers/api";
import { DeletePermanentWithdraw } from "@/types/domain/request";
import {
  FindAllWithdrawTrashed,
  RestoreWithdrawTrashed,
} from "@/types/domain/request";
import {
  ApiResponsePaginationWithdrawDeleteAt,
  ApiResponseWithdraw,
  ApiResponseWithdrawAll,
  ApiResponseWithdrawDelete,
} from "@/types/domain/response";

class WithdrawTrashedService {
  async findAllWithdrawsTrashed(
    req: FindAllWithdrawTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationWithdrawDeleteAt> {
    try {
      const response = await myApi.get("/withdraws/trashed", {
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
  async restoreWithdrawTrashed(
    req: RestoreWithdrawTrashed,
    access_token: string,
  ): Promise<ApiResponseWithdraw["data"]> {
    try {
      const response = await myApi.post(`/withdraws/restore/${req.id}`, {
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
  async deletePermanentWithdraw(
    req: DeletePermanentWithdraw,
    access_token: string,
  ): Promise<ApiResponseWithdrawDelete> {
    try {
      const response = await myApi.delete(`/withdraws/delete/${req.id}`, {
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
  async restoreWithdrawAllTrashed(
    access_token: string,
  ): Promise<ApiResponseWithdrawAll> {
    try {
      const response = await myApi.post(`/withdraws/restore-all`, null, {
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
  async deletePermanentAllWithdraw(
    access_token: string,
  ): Promise<ApiResponseWithdrawAll> {
    try {
      const response = await myApi.post(`/withdraws/delete-all`, null, {
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

export default new WithdrawTrashedService();
