import {
  CreateTransaction,
  FindAllTransaction,
  FindyByCardNumberTransaction,
  FindyByIdTransaction,
  FindyByMerchantTransaction,
  TrashedTransaction,
  UpdateTransaction,
} from "@/types/domain/request";
import {
  ApiResponsePaginationTransaction,
  ApiResponsePaginationTransactionDeleteAt,
  ApiResponseTransaction,
  ApiResponseTransactionMonthAmount,
  ApiResponseTransactionMonthMethod,
  ApiResponseTransactionMonthStatusFailed,
  ApiResponseTransactionMonthStatusSuccess,
  ApiResponseTransactions,
  ApiResponseTransactionYearAmount,
  ApiResponseTransactionYearMethod,
  ApiResponseTransactionYearStatusFailed,
  ApiResponseTransactionYearStatusSuccess,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class TransactionCommand {
  async findMonthStatusSuccessTransaction(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTransactionMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success_transaction", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccessTransaction:", error);
      throw error;
    }
  }

  async findYearStatusSuccessTransaction(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransactionYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success_transaction", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccessTransaction:", error);
      throw error;
    }
  }

  async findMonthStatusFailedTransaction(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTransactionMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed_transaction", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailedTransaction:", error);
      throw error;
    }
  }

  async findYearStatusFailedTransaction(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransactionYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed_transaction", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailedTransaction:", error);
      throw error;
    }
  }

  async findMonthStatusSuccessTransactionByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionMonthStatusSuccess> {
    try {
      return await invoke(
        "find_month_status_success_transaction_by_card_number",
        {
          accessToken,
          year,
          month,
          cardNumber,
        },
      );
    } catch (error) {
      console.error("Error in findMonthStatusSuccessTransaction:", error);
      throw error;
    }
  }

  async findYearStatusSuccessTransactionByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionYearStatusSuccess> {
    try {
      return await invoke(
        "find_year_status_success_transaction_by_card_number",
        {
          accessToken,
          year,
          cardNumber,
        },
      );
    } catch (error) {
      console.error("Error in findYearStatusSuccessTransaction:", error);
      throw error;
    }
  }

  async findMonthStatusFailedTransactionByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionMonthStatusFailed> {
    try {
      return await invoke(
        "find_month_status_failed_transaction_by_card_number",
        {
          accessToken,
          year,
          month,
          cardNumber,
        },
      );
    } catch (error) {
      console.error("Error in findMonthStatusFailedTransaction:", error);
      throw error;
    }
  }

  async findYearStatusFailedTransactionByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionYearStatusFailed> {
    try {
      return await invoke(
        "find_year_status_failed_transaction_by_card_number",
        {
          accessToken,
          year,
          cardNumber,
        },
      );
    } catch (error) {
      console.error("Error in findYearStatusFailedTransaction:", error);
      throw error;
    }
  }

  async findMonthTransactionMethod(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransactionMonthMethod> {
    try {
      return await invoke("find_month_transaction_method", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTransactionMethod:", error);
      throw error;
    }
  }

  async findYearTransactionMethod(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransactionYearMethod> {
    try {
      return await invoke("find_year_transaction_method", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTransactionMethod:", error);
      throw error;
    }
  }

  async findMonthTransactionAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransactionMonthAmount> {
    try {
      return await invoke("find_month_transaction_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTransactionAmount:", error);
      throw error;
    }
  }

  async findYearTransactionAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTransactionYearAmount> {
    try {
      return await invoke("find_year_transaction_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTransactionAmount:", error);
      throw error;
    }
  }

  async findMonthTransactionMethodByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionMonthMethod> {
    try {
      return await invoke("find_month_transaction_method_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTransactionMethodByCard:", error);
      throw error;
    }
  }

  async findYearTransactionMethodByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionYearMethod> {
    try {
      return await invoke("find_year_transaction_method_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransactionMethodByCard:", error);
      throw error;
    }
  }

  async findMonthTransactionAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionMonthAmount> {
    try {
      return await invoke("find_month_transaction_amount_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTransactionAmountByCard:", error);
      throw error;
    }
  }

  async findYearTransactionAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTransactionYearAmount> {
    try {
      return await invoke("find_year_transaction_amount_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransactionAmountByCard:", error);
      throw error;
    }
  }

  async findAllTransactions(
    accessToken: string,
    req: FindAllTransaction,
  ): Promise<ApiResponsePaginationTransaction> {
    try {
      return await invoke("find_all_transactions_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTransactions:", error);
      throw error;
    }
  }

  async findByIdTransaction(
    accessToken: string,
    req: FindyByIdTransaction,
  ): Promise<ApiResponseTransaction> {
    try {
      return await invoke("find_by_id_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdTransaction:", error);
      throw error;
    }
  }

  async findByCardNumberTransaction(
    accessToken: string,
    req: FindyByCardNumberTransaction,
  ): Promise<ApiResponsePaginationTransaction> {
    try {
      return await invoke("find_by_card_number_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByCardNumberTransaction:", error);
      throw error;
    }
  }

  async findByMerchantTransaction(
    accessToken: string,
    req: FindyByMerchantTransaction,
  ): Promise<ApiResponseTransactions> {
    try {
      return await invoke("find_by_merchant_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByMerchantTransaction:", error);
      throw error;
    }
  }

  async findByActiveTransaction(
    accessToken: string,
    req: FindAllTransaction,
  ): Promise<ApiResponsePaginationTransactionDeleteAt> {
    try {
      return await invoke("find_by_active_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActiveTransaction:", error);
      throw error;
    }
  }

  async createTransaction(
    accessToken: string,
    req: CreateTransaction,
  ): Promise<ApiResponseTransaction> {
    try {
      return await invoke("create_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createTransaction:", error);
      throw error;
    }
  }

  async updateTransaction(
    accessToken: string,
    req: UpdateTransaction,
  ): Promise<ApiResponseTransaction> {
    try {
      return await invoke("update_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateTransaction:", error);
      throw error;
    }
  }

  async trashedTransaction(
    accessToken: string,
    req: TrashedTransaction,
  ): Promise<ApiResponseTransaction> {
    try {
      return await invoke("trashed_transaction", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedTransaction:", error);
      throw error;
    }
  }
}

export default new TransactionCommand();
