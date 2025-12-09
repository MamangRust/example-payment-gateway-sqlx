import myApi from "@/helpers/api";
import {
  DeletePermanentSaldo,
  FindAllSaldoTrashed,
  RestoreSaldoTrashed,
} from "@/types/domain/request";
import {
  ApiResponsePaginationSaldoDeleteAt,
  ApiResponseSaldo,
  ApiResponseSaldoAll,
  ApiResponseSaldoDelete,
} from "@/types/domain/response";

class SaldoTrashedService {
  async findAllSaldosTrashed(
    req: FindAllSaldoTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationSaldoDeleteAt> {
    try {
      const response = await myApi.get("/saldos/trashed", {
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
  async restoreSaldoTrashed(
    req: RestoreSaldoTrashed,
    access_token: string,
  ): Promise<ApiResponseSaldo["data"]> {
    try {
      const response = await myApi.post(`/saldos/restore/${req.id}`, null, {
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
  async deletePermanentSaldo(
    req: DeletePermanentSaldo,
    access_token: string,
  ): Promise<ApiResponseSaldoDelete> {
    try {
      const response = await myApi.post(`/saldos/delete/${req.id}`, null, {
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
  async restoreSaldoAllTrashed(
    access_token: string,
  ): Promise<ApiResponseSaldoAll> {
    try {
      const response = await myApi.post(`/saldos/restore-all`, null, {
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
  async deletePermanentAllSaldo(
    access_token: string,
  ): Promise<ApiResponseSaldoAll> {
    try {
      const response = await myApi.post(`/saldos/delete-all`, null, {
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

export default new SaldoTrashedService();
