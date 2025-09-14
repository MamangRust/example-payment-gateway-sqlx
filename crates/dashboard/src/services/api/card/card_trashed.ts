import myApi from "@/helpers/api";
import {
  DeletePermanentCard,
  FindAllTrashedCard,
  RestoreTrashedCard,
} from "@/types/domain/request";
import {
  ApiResponseCard,
  ApiResponseCardAll,
  ApiResponseCardDelete,
  ApiResponsePaginationCardDeleteAt,
} from "@/types/domain/response";

class CardTrashedService {
  async findAllCardsTrashed(
    req: FindAllTrashedCard,
    access_token: string,
  ): Promise<ApiResponsePaginationCardDeleteAt> {
    try {
      const response = await myApi.get("/cards/trashed", {
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
  async restoreCardTrashed(
    req: RestoreTrashedCard,
    access_token: string,
  ): Promise<ApiResponseCard["data"]> {
    try {
      const response = await myApi.post(`/cards/restore/${req.id}`, null, {
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
  async deletePermanentCard(
    req: DeletePermanentCard,
    access_token: string,
  ): Promise<ApiResponseCardDelete> {
    try {
      const response = await myApi.post(`/cards/delete/${req.id}`, null, {
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
  async restoreCardAllTrashed(
    access_token: string,
  ): Promise<ApiResponseCardAll> {
    try {
      const response = await myApi.post(`/cards/restore-all`, null, {
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
  async deletePermanentAllCard(
    access_token: string,
  ): Promise<ApiResponseCardAll> {
    try {
      const response = await myApi.post(`/cards/delete-all`, null, {
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

export default new CardTrashedService();
