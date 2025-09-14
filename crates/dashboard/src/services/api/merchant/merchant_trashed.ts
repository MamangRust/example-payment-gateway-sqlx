import myApi from "@/helpers/api";
import {
  DeletePermanentMerchant,
  FindAllMerchantTrashed,
  RestoreMerchantTrashed,
} from "@/types/domain/request";
import {
  ApiResponseMerchant,
  ApiResponseMerchantAll,
  ApiResponseMerchantDelete,
  ApiResponsePaginationMerchantDeleteAt,
} from "@/types/domain/response";

class MerchantTrashedService {
  async findAllMerchantsTrashed(
    req: FindAllMerchantTrashed,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchantDeleteAt> {
    try {
      const response = await myApi.get("/merchants/trashed", {
        params: { page: req.page, page_size: req.pageSize, search: req.search },
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
  async restoreMerchantTrashed(
    req: RestoreMerchantTrashed,
    access_token: string,
  ): Promise<ApiResponseMerchant["data"]> {
    try {
      const response = await myApi.post(`/merchants/restore/${req.id}`, null, {
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
  async deletePermanentMerchant(
    req: DeletePermanentMerchant,
    access_token: string,
  ): Promise<ApiResponseMerchantDelete> {
    try {
      const response = await myApi.post(
        `/merchants/delete/${req.id}`,
        null,
        {
          headers: { Authorization: `Bearer ${access_token}` },
        },
      );

      if (response.status == 200) {
        return response.data.data;
      }

      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async restoreMerchantAllTrashed(
    access_token: string,
  ): Promise<ApiResponseMerchantAll> {
    try {
      const response = await myApi.post(`/merchants/restore-all`, null, {
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
  async deletePermanentAllMerchant(
    access_token: string,
  ): Promise<ApiResponseMerchantAll> {
    try {
      const response = await myApi.post(`/merchants/delete-all`, null, {
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

export default new MerchantTrashedService();
