import myApi from "@/helpers/api";
import { DeletePermanentTransaction } from "@/types/domain/request";
import {
  FindAllTransactionTrashed,
  RestoreTransactionTrashed,
} from "@/types/domain/request";
import {
  ApiResponsePaginationTransactionDeleteAt,
  ApiResponseTransaction,
  ApiResponseTransactionAll,
} from "@/types/domain/response";

class TransactionTrashedService {
  async findAllTransactionsTrashed(
    req: FindAllTransactionTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationTransactionDeleteAt> {
    try {
      const response = await myApi.get("/transactions/trashed", {
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
  async restoreTransactionTrashed(
    req: RestoreTransactionTrashed,
    access_token: string,
  ): Promise<ApiResponseTransaction["data"]> {
    try {
      const response = await myApi.post(`/transactions/restore/${req.id}`, {
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
  async deletePermanentTransaction(
    req: DeletePermanentTransaction,
    access_token: string,
  ): Promise<ApiResponseTransaction> {
    try {
      const response = await myApi.delete(`/transactions/delete/${req.id}`, {
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
  async restoreTransactionAllTrashed(
    access_token: string,
  ): Promise<ApiResponseTransactionAll> {
    try {
      const response = await myApi.post(`/transactions/restore-all`, null, {
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
  async deletePermanentAllTransaction(
    access_token: string,
  ): Promise<ApiResponseTransactionAll> {
    try {
      const response = await myApi.post(`/transactions/delete-all`, null, {
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

export default new TransactionTrashedService();
