import { invoke } from "@tauri-apps/api/core";
import {
  ApiResponseCard,
  ApiResponseDashboardCard,
  ApiResponseDashboardCardNumber,
  ApiResponseMonthlyBalance,
  ApiResponseMonthlyTopupAmount,
  ApiResponseMonthlyTransactionAmount,
  ApiResponseMonthlyTransferAmount,
  ApiResponseMonthlyWithdrawAmount,
  ApiResponsePaginationCard,
  ApiResponsePaginationCardDeleteAt,
  ApiResponseYearlyBalance,
  ApiResponseYearlyTopupAmount,
  ApiResponseYearlyTransactionAmount,
  ApiResponseYearlyTransferAmount,
  ApiResponseYearlyWithdrawAmount,
} from "@/types/domain/response";
import {
  CreateCard,
  FindAllCard,
  FindByCardNumber,
  FindByIdCard,
  UpdateCard,
} from "@/types/domain/request";
import { FindByUser } from "@/types/domain/request/card/user";

class CardCommand {
  async findDashboard(accessToken: string): Promise<ApiResponseDashboardCard> {
    try {
      return await invoke<ApiResponseDashboardCard>("find_dashboard", {
        accessToken,
      });
    } catch (error) {
      console.error("Error in findDashboard:", error);
      throw error;
    }
  }

  async findDashboardByCardNumber(
    accessToken: string,
    cardNumber: string,
  ): Promise<ApiResponseDashboardCardNumber> {
    try {
      return await invoke("find_dashboard_by_card_number", {
        accessToken,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findDashboardByCardNumber:", error);
      throw error;
    }
  }

  async findMonthBalance(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthlyBalance> {
    try {
      return await invoke("find_month_balance_card", { accessToken, year });
    } catch (error) {
      console.error("Error in findMonthBalanceCard:", error);
      throw error;
    }
  }

  async findYearBalance(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearlyBalance> {
    try {
      return await invoke("find_year_balance_card", { accessToken, year });
    } catch (error) {
      console.error("Error in findYearBalanceCard:", error);
      throw error;
    }
  }

  async findMonthTopupAmountCard(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthlyTopupAmount> {
    try {
      return await invoke("find_month_topup_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTopupAmountCard:", error);
      throw error;
    }
  }

  async findYearTopupAmountCard(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearlyTopupAmount> {
    try {
      return await invoke("findyear_topup_amount_card", { accessToken, year });
    } catch (error) {
      console.error("Error in findYearTopupAmountCard:", error);
      throw error;
    }
  }

  async findMonthWithdrawAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthlyWithdrawAmount> {
    try {
      return await invoke("find_month_withdraw_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthWithdrawAmountCard:", error);
      throw error;
    }
  }

  async findYearWithdrawAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearlyWithdrawAmount> {
    try {
      return await invoke("findyear_withdraw_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearWithdrawAmountCard:", error);
      throw error;
    }
  }

  async findMonthTransferSenderAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthlyTransferAmount> {
    try {
      return await invoke("find_month_transfer_sender_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTransferSenderAmountCard:", error);
      throw error;
    }
  }

  async findYearTransferSenderAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearlyTransferAmount> {
    try {
      return await invoke("findyear_transfer_sender_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTransferSenderAmountCard:", error);
      throw error;
    }
  }

  async findMonthTransferReceiverAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthlyTransferAmount> {
    try {
      return await invoke("find_month_transfer_receiver_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTransferReceiverAmountCard:", error);
      throw error;
    }
  }

  async findYearTransferReceiverAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearlyTransferAmount> {
    try {
      return await invoke("findyear_transfer_receiver_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTransferReceiverAmountCard:", error);
      throw error;
    }
  }

  async findMonthTransactionAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseMonthlyTransactionAmount> {
    try {
      return await invoke("find_month_transaction_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTransactionAmountCard:", error);
      throw error;
    }
  }

  async findYearTransactionAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseYearlyTransactionAmount> {
    try {
      return await invoke("findyear_transaction_amount_card", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTransactionAmountCard:", error);
      throw error;
    }
  }

  async findMonthBalanceByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseMonthlyBalance> {
    try {
      return await invoke("find_month_balance_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthBalanceByCard:", error);
      throw error;
    }
  }

  async findYearBalanceByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseYearlyBalance> {
    try {
      return await invoke("find_year_balance_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearBalanceByCard:", error);
      throw error;
    }
  }

  async findMonthTopupAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseMonthlyTopupAmount> {
    try {
      return await invoke("find_month_topup_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTopupAmountByCard:", error);
      throw error;
    }
  }

  async findYearTopupAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseYearlyTopupAmount> {
    try {
      return await invoke("find_year_topup_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTopupAmountByCard:", error);
      throw error;
    }
  }

  async findMonthWithdrawAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseMonthlyWithdrawAmount> {
    try {
      return await invoke("find_month_withdraw_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthWithdrawAmountByCard:", error);
      throw error;
    }
  }

  async findYearWithdrawAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseYearlyWithdrawAmount> {
    try {
      return await invoke("findyear_withdraw_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearWithdrawAmountByCard:", error);
      throw error;
    }
  }

  async findMonthTransferSenderAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseMonthlyTransferAmount> {
    try {
      return await invoke("find_month_transfer_sender_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTransferSenderAmountByCard:", error);
      throw error;
    }
  }

  async findYearTransferSenderAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseYearlyTransferAmount> {
    try {
      return await invoke("find_year_transfer_sender_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransferSenderAmountByCard:", error);
      throw error;
    }
  }

  async findMonthTransferReceiverAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseMonthlyTransferAmount> {
    try {
      return await invoke("find_month_transfer_receiver_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTransferReceiverAmountByCard:", error);
      throw error;
    }
  }

  async findYearTransferReceiverAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseYearlyTransferAmount> {
    try {
      return await invoke("find_year_transfer_receiver_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransferReceiverAmountByCard:", error);
      throw error;
    }
  }

  async findMonthTransactionAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseMonthlyTransactionAmount> {
    try {
      return await invoke("find_month_transaction_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
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
  ): Promise<ApiResponseYearlyTransactionAmount> {
    try {
      return await invoke("findyear_transaction_amount_by_card_card", {
        access_token: accessToken,
        year,
        card_number: cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTransactionAmountByCard:", error);
      throw error;
    }
  }

  async findAllCard(
    accessToken: string,
    req: FindAllCard,
  ): Promise<ApiResponsePaginationCard> {
    try {
      return await invoke("find_all_card", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllCard:", error);
      throw error;
    }
  }

  async findByIdCard(
    accessToken: string,
    req: FindByIdCard,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("find_by_id_card", { accessToken, req });
    } catch (error) {
      console.error("Error in findByIdCard:", error);
      throw error;
    }
  }

  async findByUser(
    accessToken: string,
    req: FindByUser,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("find_by_user", { accessToken, req });
    } catch (error) {
      console.error("Error in findByUser:", error);
      throw error;
    }
  }

  async findByCardNumber(
    accessToken: string,
    req: FindByCardNumber,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("find_by_card_number", { accessToken, req });
    } catch (error) {
      console.error("Error in findByCardNumber:", error);
      throw error;
    }
  }

  async findActiveCard(
    accessToken: string,
    req: FindAllCard,
  ): Promise<ApiResponsePaginationCardDeleteAt> {
    try {
      return await invoke("find_active_card", { accessToken, req });
    } catch (error) {
      console.error("Error in findActiveCard:", error);
      throw error;
    }
  }

  async createCard(
    accessToken: string,
    req: CreateCard,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("create_card", { accessToken, req });
    } catch (error) {
      console.error("Error in createCard:", error);
      throw error;
    }
  }

  async updateCard(
    accessToken: string,
    req: UpdateCard,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("update_card", { accessToken, req });
    } catch (error) {
      console.error("Error in updateCard:", error);
      throw error;
    }
  }

  async trashedCard(
    accessToken: string,
    req: FindByIdCard,
  ): Promise<ApiResponseCard> {
    try {
      return await invoke("trashed_card", { accessToken, req });
    } catch (error) {
      console.error("Error in trashedCard:", error);
      throw error;
    }
  }
}

export default new CardCommand();
