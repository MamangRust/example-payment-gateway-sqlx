import myApi from "@/helpers/api";
import {
  CreateMerchant,
  FindAllMerchant,
  FindAllMerchantTransaction,
  FindAllTransactionByApiKey,
  findByApiKeyMerchant,
  FindByIdMerchant,
  FindMerchantUser,
  FindTrashedMerchant,
  UpdateMerchant,
} from "@/types/domain/request";
import {
  ApiResponseMerchant,
  ApiResponseMerchantMonthlyAmount,
  ApiResponseMerchantMonthlyPaymentMethod,
  ApiResponseMerchantMonthlyTotalAmount,
  ApiResponseMerchantYearlyAmount,
  ApiResponseMerchantYearlyPaymentMethod,
  ApiResponseMerchantYearlyTotalAmount,
  ApiResponsePaginationMerchant,
  ApiResponsePaginationMerchantTransaction,
  ApiResponsesMerchant,
} from "@/types/domain/response";

class MerchantService {
  async findMonthPaymentMethod(
    access_token: string,
    year: number,
  ): Promise<ApiResponseMerchantMonthlyPaymentMethod["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/method/monthly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findYearPaymentMethod(
    access_token: string,
    year: number,
  ): Promise<ApiResponseMerchantYearlyPaymentMethod["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/method/yearly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });
      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findMonthAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseMerchantMonthlyAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/amount/monthly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });
      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findYearAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseMerchantYearlyAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/amount/yearly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findMonthTotalAmount(
    access_token: string,
    year: number,
    month: number,
  ): Promise<ApiResponseMerchantMonthlyTotalAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/total-amount/monthly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          month,
        },
      });
      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findYearTotalAmount(
    access_token: string,
    year: number,
  ): Promise<ApiResponseMerchantYearlyTotalAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/total-amount/yearly", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findMonthPaymentMethodByMerchant(
    access_token: string,
    year: number,
    merchant_id: number,
  ): Promise<ApiResponseMerchantMonthlyPaymentMethod["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/method/monthly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            merchant_id,
          },
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

  async findYearPaymentMethodByMerchant(
    access_token: string,
    year: number,
    merchant_id: number,
  ): Promise<ApiResponseMerchantYearlyPaymentMethod["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/method/yearly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            merchant_id,
          },
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

  async findMonthAmountByMerchant(
    access_token: string,
    year: number,
    merchant_id: number,
  ): Promise<ApiResponseMerchantMonthlyAmount["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/amount/monthly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            merchant_id,
          },
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

  async findYearAmountByMerchant(
    access_token: string,
    year: number,
    merchant_id: number,
  ): Promise<ApiResponseMerchantYearlyAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/amount/yearly/by-merchant", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          merchant_id,
        },
      });
      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findMonthTotalAmountByMerchant(
    access_token: string,
    year: number,
    month: number,
    merchant_id: number,
  ): Promise<ApiResponseMerchantMonthlyTotalAmount["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/total-amount/monthly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            month,
            merchant_id,
          },
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

  async findYearTotalAmountByMerchant(
    access_token: string,
    year: number,
    merchant_id: number,
  ): Promise<ApiResponseMerchantYearlyTotalAmount["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/total-amount/yearly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            merchant_id,
          },
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

  async findMonthPaymentMethodByApiKey(
    access_token: string,
    year: number,
    api_key: string,
  ): Promise<ApiResponseMerchantMonthlyPaymentMethod["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/method/monthly/by-apikey",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            api_key,
          },
        },
      );
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Request failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Request failed.");
    }
  }

  async findYearPaymentMethodByApiKey(
    access_token: string,
    year: number,
    api_key: string,
  ): Promise<ApiResponseMerchantYearlyPaymentMethod["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/method/yearly/by-apikey",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            api_key,
          },
        },
      );

      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Request failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Request failed.");
    }
  }

  async findMonthAmountByApiKey(
    access_token: string,
    year: number,
    api_key: string,
  ): Promise<ApiResponseMerchantMonthlyAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/amount/monthly/by-apikey", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          api_key,
        },
      });
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Request failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Request failed.");
    }
  }

  async findYearAmountByApiKey(
    access_token: string,
    year: number,
    api_key: string,
  ): Promise<ApiResponseMerchantYearlyAmount["data"]> {
    try {
      const response = await myApi.get("/merchants/stats/amount/yearly/by-apikey", {
        headers: { Authorization: `Bearer ${access_token}` },
        params: {
          year,
          api_key,
        },
      });
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Request failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Request failed.");
    }
  }

  async findMonthTotalAmountByApiKey(
    access_token: string,
    year: number,
    month: number,
    api_key: string,
  ): Promise<ApiResponseMerchantMonthlyTotalAmount["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/total-amount/monthly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            month,
            api_key,
          },
        },
      );
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Request failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Request failed.");
    }
  }

  async findYearTotalAmountByApiKey(
    access_token: string,
    year: number,
    api_key: string,
  ): Promise<ApiResponseMerchantYearlyTotalAmount["data"]> {
    try {
      const response = await myApi.get(
        "/merchants/stats/total-amount/yearly/by-merchant",
        {
          headers: { Authorization: `Bearer ${access_token}` },
          params: {
            year,
            api_key,
          },
        },
      );
      if (response.status === 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Request failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Request failed.");
    }
  }

  async findAllMerchants(
    req: FindAllMerchant,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchant> {
    try {
      const response = await myApi.get("/merchants", {
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
  async findAllTransaction(
    req: FindAllMerchant,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchantTransaction> {
    try {
      const response = await myApi.get("/merchants/transactions", {
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

  async findAllTransactionByMerchant(
    req: FindAllMerchantTransaction,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchantTransaction> {
    try {
      const response = await myApi.get("/merchants/transactions/by-merchant", {
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

  async findAllTransactionByApiKey(
    req: FindAllTransactionByApiKey,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchantTransaction> {
    try {
      const response = await myApi.get(
        "/merchants/transactions/by-apikey/" + req.api_key,
        {
          params: {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
          },
          headers: { Authorization: `Bearer ${access_token}` },
        },
      );

      if (response.status == 200) {
        return response.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findById(
    req: FindByIdMerchant,
    access_token: string,
  ): Promise<ApiResponseMerchant["data"]> {
    try {
      const response = await myApi.get(`/merchants/${req.id}`, {
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

  async findByApiKey(
    req: findByApiKeyMerchant,
    access_token: string,
  ): Promise<ApiResponseMerchant["data"]> {
    try {
      const response = await myApi.get(`/merchants/by-apikey/${req.api_key}`, {
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

  async findByMerchantUser(
    req: FindMerchantUser,
    access_token: string,
  ): Promise<ApiResponsesMerchant["data"]> {
    try {
      const response = await myApi.get(
        `/merchants/by-user/${req.user_id}`,
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

  async findByActive(
    req: FindAllMerchant,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchant> {
    try {
      const response = await myApi.get("/merchants/active", {
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

  async createMerchant(
    req: CreateMerchant,
    access_token: string,
  ): Promise<ApiResponseMerchant["data"]> {
    try {
      const response = await myApi.post(
        "/merchants/create",
        {
          name: req.name,
          user_id: req.user_id,
        },
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

  async updateMerchant(
    req: UpdateMerchant,
    access_token: string,
  ): Promise<ApiResponseMerchant["data"]> {
    try {
      const response = await myApi.post(
        `/merchants/update/${req.merchant_id}`,
        {
          merchant_id: req.merchant_id,
          name: req.name,
          user_id: req.user_id,
          status: req.status,
        },
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

  async trashedMerchant(
    req: FindTrashedMerchant,
    access_token: string,
  ): Promise<ApiResponsePaginationMerchant["data"]> {
    try {
      const response = await myApi.post(`/merchants/trash/${req.id}`, null, {
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

export default new MerchantService();
