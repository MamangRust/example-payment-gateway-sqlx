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
  ApiResponsePaginationMerchantDeleteAt,
  ApiResponsePaginationMerchantTransaction,
  ApiResponsesMerchant,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class MerchantCommandService {
  async findMonthPaymentMethod(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMerchantMonthlyPaymentMethod> {
    try {
      return await invoke("find_month_payment_method", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthPaymentMethod:", error);
      throw error;
    }
  }

  async findYearPaymentMethod(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMerchantYearlyPaymentMethod> {
    try {
      return await invoke("find_year_payment_method", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearPaymentMethod:", error);
      throw error;
    }
  }

  async findMonthAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMerchantMonthlyAmount> {
    try {
      return await invoke("find_month_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthAmount:", error);
      throw error;
    }
  }

  async findYearAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMerchantYearlyAmount> {
    try {
      return await invoke("find_year_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearAmount:", error);
      throw error;
    }
  }

  async findMonthTotalAmount(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseMerchantMonthlyTotalAmount> {
    try {
      return await invoke("find_month_total_amount", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthTotalAmount:", error);
      throw error;
    }
  }

  async findYearTotalAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMerchantYearlyTotalAmount> {
    try {
      return await invoke("find_year_total_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTotalAmount:", error);
      throw error;
    }
  }

  async findMonthPaymentMethodByMerchant(
    accessToken: string,
    year: number,
    merchantId: number,
  ): Promise<ApiResponseMerchantMonthlyPaymentMethod> {
    try {
      return await invoke("find_month_payment_method_by_merchant", {
        accessToken,
        year,
        merchantId,
      });
    } catch (error) {
      console.error("Error in findMonthPaymentMethodByMerchant:", error);
      throw error;
    }
  }

  async findYearPaymentMethodByMerchant(
    accessToken: string,
    year: number,
    merchantId: number,
  ): Promise<ApiResponseMerchantYearlyPaymentMethod> {
    try {
      return await invoke("find_year_payment_method_by_merchant", {
        accessToken,
        year,
        merchantId,
      });
    } catch (error) {
      console.error("Error in findYearPaymentMethodByMerchant:", error);
      throw error;
    }
  }

  async findMonthAmountByMerchant(
    accessToken: string,
    year: number,
    merchantId: number,
  ): Promise<ApiResponseMerchantMonthlyAmount> {
    try {
      return await invoke("find_month_amount_by_merchant", {
        accessToken,
        year,
        merchantId,
      });
    } catch (error) {
      console.error("Error in findMonthAmountByMerchant:", error);
      throw error;
    }
  }

  async findYearAmountByMerchant(
    accessToken: string,
    year: number,
    merchantId: number,
  ): Promise<ApiResponseMerchantYearlyAmount> {
    try {
      return await invoke("find_year_amount_by_merchant", {
        accessToken,
        year,
        merchantId,
      });
    } catch (error) {
      console.error("Error in findYearAmountByMerchant:", error);
      throw error;
    }
  }

  async findMonthTotalAmountByMerchant(
    accessToken: string,
    year: number,
    month: number,
    merchantId: number,
  ): Promise<ApiResponseMerchantMonthlyTotalAmount> {
    try {
      return await invoke("find_month_total_amount_by_merchant", {
        accessToken,
        year,
        month,
        merchantId,
      });
    } catch (error) {
      console.error("Error in findMonthTotalAmountByMerchant:", error);
      throw error;
    }
  }

  async findYearTotalAmountByMerchant(
    accessToken: string,
    year: number,
    merchantId: number,
  ): Promise<ApiResponseMerchantYearlyTotalAmount> {
    try {
      return await invoke("find_year_total_amount_by_merchant", {
        accessToken,
        year,
        merchantId,
      });
    } catch (error) {
      console.error("Error in findYearTotalAmountByMerchant:", error);
      throw error;
    }
  }

  async findMonthPaymentMethodByApiKey(
    accessToken: string,
    year: number,
    apiKey: string,
  ): Promise<ApiResponseMerchantMonthlyPaymentMethod> {
    try {
      return await invoke("find_month_payment_method_by_api_key", {
        accessToken,
        year,
        apiKey,
      });
    } catch (error) {
      console.error("Error in findMonthPaymentMethodByApiKey:", error);
      throw error;
    }
  }

  async findYearPaymentMethodByApiKey(
    accessToken: string,
    year: number,
    apiKey: string,
  ): Promise<ApiResponseMerchantYearlyPaymentMethod> {
    try {
      return await invoke("find_year_payment_method_by_api_key", {
        accessToken,
        year,
        apiKey,
      });
    } catch (error) {
      console.error("Error in findYearPaymentMethodByApiKey:", error);
      throw error;
    }
  }

  async findMonthAmountByApiKey(
    accessToken: string,
    year: number,
    apiKey: string,
  ): Promise<ApiResponseMerchantMonthlyAmount> {
    try {
      return await invoke("find_month_amount_by_api_key", {
        accessToken,
        year,
        apiKey,
      });
    } catch (error) {
      console.error("Error in findMonthAmountByApiKey:", error);
      throw error;
    }
  }

  async findYearAmountByApiKey(
    accessToken: string,
    year: number,
    apiKey: string,
  ): Promise<ApiResponseMerchantYearlyAmount> {
    try {
      return await invoke("find_year_amount_by_api_key", {
        accessToken,
        year,
        apiKey,
      });
    } catch (error) {
      console.error("Error in findYearAmountByApiKey:", error);
      throw error;
    }
  }

  async findMonthTotalAmountByApiKey(
    accessToken: string,
    year: number,
    month: number,
    apiKey: string,
  ): Promise<ApiResponseMerchantMonthlyTotalAmount> {
    try {
      return await invoke("find_month_total_amount_by_api_key", {
        accessToken,
        year,
        month,
        apiKey,
      });
    } catch (error) {
      console.error("Error in findMonthTotalAmountByApiKey:", error);
      throw error;
    }
  }

  async findYearTotalAmountByApiKey(
    accessToken: string,
    year: number,
    apiKey: string,
  ): Promise<ApiResponseMerchantYearlyTotalAmount> {
    try {
      return await invoke("find_year_total_amount_by_api_key", {
        accessToken,
        year,
        apiKey,
      });
    } catch (error) {
      console.error("Error in findYearTotalAmountByApiKey:", error);
      throw error;
    }
  }

  async findAllMerchants(
    accessToken: string,
    req: FindAllMerchant,
  ): Promise<ApiResponsePaginationMerchant> {
    try {
      return await invoke("find_all_merchants", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllMerchants:", error);
      throw error;
    }
  }

  async findAllTransactions(
    accessToken: string,
    req: FindAllMerchant,
  ): Promise<ApiResponsePaginationMerchantTransaction> {
    try {
      return await invoke("find_all_transactions", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTransactions:", error);
      throw error;
    }
  }

  async findAllTransactionByMerchant(
    accessToken: string,
    req: FindAllMerchantTransaction,
  ): Promise<ApiResponsePaginationMerchantTransaction> {
    try {
      return await invoke("find_all_transactions_by_merchant", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTransactions:", error);
      throw error;
    }
  }
  async findAllTransactionByApiKey(
    accessToken: string,
    req: FindAllTransactionByApiKey,
  ): Promise<ApiResponsePaginationMerchantTransaction> {
    try {
      return await invoke("find_all_transactions_by_api_key", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTransactions:", error);
      throw error;
    }
  }

  async findMerchantById(
    accessToken: string,
    req: FindByIdMerchant,
  ): Promise<ApiResponseMerchant> {
    try {
      return await invoke("find_by_id", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findMerchantById:", error);
      throw error;
    }
  }

  async findMerchantByApiKey(
    accessToken: string,
    req: findByApiKeyMerchant,
  ): Promise<ApiResponseMerchant> {
    try {
      return await invoke("find_by_api_key", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findMerchantByApiKey:", error);
      throw error;
    }
  }

  async findMerchantByUser(
    accessToken: string,
    req: FindMerchantUser,
  ): Promise<ApiResponsesMerchant> {
    try {
      return await invoke("find_by_merchant_user", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findMerchantByUser:", error);
      throw error;
    }
  }

  async findActiveMerchant(
    accessToken: string,
    req: FindAllMerchant,
  ): Promise<ApiResponsePaginationMerchantDeleteAt> {
    try {
      return await invoke("find_by_active_merchant", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findActiveMerchant:", error);
      throw error;
    }
  }

  async createMerchant(
    accessToken: string,
    req: CreateMerchant,
  ): Promise<ApiResponseMerchant> {
    try {
      return await invoke("create_merchant", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createMerchant:", error);
      throw error;
    }
  }

  async updateMerchant(
    accessToken: string,
    req: UpdateMerchant,
  ): Promise<ApiResponseMerchant> {
    try {
      return await invoke("update_merchant", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateMerchant:", error);
      throw error;
    }
  }

  async trashedMerchant(
    accessToken: string,
    req: FindTrashedMerchant,
  ): Promise<ApiResponseMerchant> {
    try {
      return await invoke("trashed_merchant", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedMerchant:", error);
      throw error;
    }
  }
}

export default new MerchantCommandService();
