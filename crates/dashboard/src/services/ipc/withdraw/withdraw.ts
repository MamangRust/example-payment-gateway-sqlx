import {
  CreateWithdraw,
  FindByIdWithdraw,
  TrashedWithdraw,
  UpdateWithdraw,
} from "@/types/domain/request";
import { FindAllWithdraw } from "@/types/domain/request/withdraw/list";
import {
  ApiResponsePaginationWithdraw,
  ApiResponsePaginationWithdrawDeleteAt,
  ApiResponseWithdraw,
  ApiResponseWithdrawMonthAmount,
  ApiResponseWithdrawMonthStatusFailed,
  ApiResponseWithdrawMonthStatusSuccess,
  ApiResponseWithdrawYearAmount,
  ApiResponseWithdrawYearStatusFailed,
  ApiResponseWithdrawYearStatusSuccess,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class WithdrawCommand {
  async findMonthStatusSuccessWithdraw(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseWithdrawMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success_withdraw", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccessWithdraw:", error);
      throw error;
    }
  }

  async findYearStatusSuccessWithdraw(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseWithdrawYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success_withdraw", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccessWithdraw:", error);
      throw error;
    }
  }

  async findMonthStatusFailedWithdraw(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseWithdrawMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed_withdraw", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailedWithdraw:", error);
      throw error;
    }
  }

  async findYearStatusFailedWithdraw(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseWithdrawYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed_withdraw", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailedWithdraw:", error);
      throw error;
    }
  }

  async findMonthStatusSuccessWithdrawByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success_withdraw_by_card_number", {
        accessToken,
        year,
        month,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccessWithdraw:", error);
      throw error;
    }
  }

  async findYearStatusSuccessWithdrawByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success_withdraw_by_card_number", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccessWithdraw:", error);
      throw error;
    }
  }

  async findMonthStatusFailedWithdrawByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed_withdraw_by_card_number", {
        accessToken,
        year,
        month,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailedWithdraw:", error);
      throw error;
    }
  }

  async findYearStatusFailedWithdrawByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed_withdraw_by_card_number", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailedWithdraw:", error);
      throw error;
    }
  }

  async findMonthWithdrawAmountWithdraw(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseWithdrawMonthAmount> {
    try {
      return await invoke("find_month_withdraw_amount_withdraw", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthWithdrawAmountWithdraw:", error);
      throw error;
    }
  }

  async findYearWithdrawAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseWithdrawYearAmount> {
    try {
      return await invoke("find_year_withdraw_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearWithdrawAmount:", error);
      throw error;
    }
  }

  async findMonthWithdrawAmountByCardWithdraw(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawMonthAmount> {
    try {
      return await invoke("find_month_withdraw_amount_by_card_withdraw", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthWithdrawAmountByCardWithdraw:", error);
      throw error;
    }
  }

  async findYearWithdrawAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseWithdrawYearAmount> {
    try {
      return await invoke("find_year_withdraw_amount_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearWithdrawAmountByCard:", error);
      throw error;
    }
  }

  async findAllWithdraws(
    accessToken: string,
    req: FindAllWithdraw,
  ): Promise<ApiResponsePaginationWithdraw> {
    try {
      const request = {
        page: req.page,
        page_size: req.page_size,
        search: req.search || "",
      };

      return await invoke("find_all_withdraws", {
        accessToken,
        req: request,
      });
    } catch (error) {
      console.error("Error in findAllWithdraws:", error);
      throw error;
    }
  }

  async findByIdWithdraw(
    accessToken: string,
    req: FindByIdWithdraw,
  ): Promise<ApiResponseWithdraw> {
    try {
      return await invoke("find_by_id_withdraw", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdWithdraw:", error);
      throw error;
    }
  }

  async findByCardNumberWithdraw(
    accessToken: string,
    cardNumber: string,
  ): Promise<ApiResponsePaginationWithdraw> {
    try {
      return await invoke("find_by_card_number_withdraw", {
        accessToken,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findByCardNumberWithdraw:", error);
      throw error;
    }
  }

  async findByActiveWithdraw(
    accessToken: string,
    req: FindAllWithdraw,
  ): Promise<ApiResponsePaginationWithdrawDeleteAt> {
    try {
      return await invoke("find_by_active_withdraw", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActiveWithdraw:", error);
      throw error;
    }
  }

  async createWithdraw(
    accessToken: string,
    req: CreateWithdraw,
  ): Promise<ApiResponseWithdraw> {
    try {
      return await invoke("create_withdraw", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createWithdraw:", error);
      throw error;
    }
  }

  async updateWithdraw(
    accessToken: string,
    req: UpdateWithdraw,
  ): Promise<ApiResponseWithdraw> {
    try {
      return await invoke("update_withdraw", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateWithdraw:", error);
      throw error;
    }
  }

  async trashedWithdraw(
    accessToken: string,
    req: TrashedWithdraw,
  ): Promise<ApiResponseWithdraw> {
    try {
      return await invoke("trashed_withdraw", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedWithdraw:", error);
      throw error;
    }
  }
}

export default new WithdrawCommand();
