import myApi from "@/helpers/api";
import {
  DeletePermanentTransfer,
  RestoreTransferTrashed,
} from "@/types/domain/request";
import { FindAllTransferTrashed } from "@/types/domain/request";
import {
  ApiResponsePaginationTransferDeleteAt,
  ApiResponseTransfers,
  ApiResponseTransferAll,
  ApiResponseTransferDelete,
} from "@/types/domain/response";

class TranferTrashedService {
  async findAllTransferssTrashed(
    req: FindAllTransferTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationTransferDeleteAt> {
    try {
      const response = await myApi.get("/transfers/trashed", {
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
  async restoreTransfersTrashed(
    req: RestoreTransferTrashed,
    access_token: string,
  ): Promise<ApiResponseTransfers["data"]> {
    try {
      const response = await myApi.post(`/transfers/restore/${req.id}`, {
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
  async deletePermanentTransfer(
    req: DeletePermanentTransfer,
    access_token: string,
  ): Promise<ApiResponseTransferDelete> {
    try {
      const response = await myApi.delete(`/transfers/delete/${req.id}`, {
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
  async restoreTransfersAllTrashed(
    access_token: string,
  ): Promise<ApiResponseTransferAll> {
    try {
      const response = await myApi.post(`/transfers/restore-all`, null, {
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
  async deletePermanentAllTransfers(
    access_token: string,
  ): Promise<ApiResponseTransferAll> {
    try {
      const response = await myApi.post(`/transfers/delete-all`, null, {
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

export default new TranferTrashedService();
