import {
  CreateTopup,
  FindAllTopup,
  FindByCardNumberTopup,
  FindByIdTopup,
  TrashedTopup,
  UpdateTopup,
} from "@/types/domain/request/topup";
import {
  ApiResponsePaginationTopup,
  ApiResponsePaginationTopupDeleteAt,
  ApiResponseTopup,
  ApiResponseTopupMonthAmount,
  ApiResponseTopupMonthMethod,
  ApiResponseTopupMonthStatusFailed,
  ApiResponseTopupMonthStatusSuccess,
  ApiResponseTopupYearAmount,
  ApiResponseTopupYearMethod,
  ApiResponseTopupYearStatusFailed,
  ApiResponseTopupYearStatusSuccess,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class TopupCommand {
  async findMonthStatusSuccess(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTopupMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccess:", error);
      throw error;
    }
  }

  async findYearStatusSuccess(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTopupYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccess:", error);
      throw error;
    }
  }

  async findMonthStatusFailed(
    accessToken: string,
    year: number,
    month: number,
  ): Promise<ApiResponseTopupMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed", {
        accessToken,
        year,
        month,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailed:", error);
      throw error;
    }
  }

  async findYearStatusFailed(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTopupYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailed:", error);
      throw error;
    }
  }

  async findMonthStatusSuccessByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupMonthStatusSuccess> {
    try {
      return await invoke("find_month_status_success_by_card_number", {
        accessToken,
        year,
        month,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthStatusSuccess:", error);
      throw error;
    }
  }

  async findYearStatusSuccessByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupYearStatusSuccess> {
    try {
      return await invoke("find_year_status_success_by_card_number", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearStatusSuccess:", error);
      throw error;
    }
  }

  async findMonthStatusFailedByCardNumber(
    accessToken: string,
    year: number,
    month: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupMonthStatusFailed> {
    try {
      return await invoke("find_month_status_failed_by_card_number", {
        accessToken,
        year,
        month,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthStatusFailed:", error);
      throw error;
    }
  }

  async findYearStatusFailedByCardNumber(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupYearStatusFailed> {
    try {
      return await invoke("find_year_status_failed_by_card_number", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearStatusFailed:", error);
      throw error;
    }
  }

  async findMonthTopupMethod(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTopupMonthMethod> {
    try {
      return await invoke("find_month_topup_method", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTopupMethod:", error);
      throw error;
    }
  }

  async findYearTopupMethod(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTopupYearMethod> {
    try {
      return await invoke("find_year_topup_method", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTopupMethod:", error);
      throw error;
    }
  }

  async findMonthTopupAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTopupMonthAmount> {
    try {
      return await invoke("find_month_topup_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findMonthTopupAmount:", error);
      throw error;
    }
  }

  async findYearTopupAmount(
    accessToken: string,
    year: number,
  ): Promise<ApiResponseTopupYearAmount> {
    try {
      return await invoke("find_year_topup_amount", {
        accessToken,
        year,
      });
    } catch (error) {
      console.error("Error in findYearTopupAmount:", error);
      throw error;
    }
  }

  async findMonthTopupMethodByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupMonthMethod> {
    try {
      return await invoke("find_month_topup_method_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findMonthTopupMethodByCard:", error);
      throw error;
    }
  }

  async findYearTopupMethodByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupYearMethod> {
    try {
      return await invoke("find_year_topup_method_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTopupMethodByCard:", error);
      throw error;
    }
  }

  async findMonthTopupAmountByCard(
    accessToken: string,
    year: number,
    cardNumber: string,
  ): Promise<ApiResponseTopupMonthAmount> {
    try {
      return await invoke("find_month_topup_amount_by_card", {
        accessToken,
        year,
        cardNumber,
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
  ): Promise<ApiResponseTopupYearAmount> {
    try {
      return await invoke("find_year_topup_amount_by_card", {
        accessToken,
        year,
        cardNumber,
      });
    } catch (error) {
      console.error("Error in findYearTopupAmountByCard:", error);
      throw error;
    }
  }

  async findAllTopups(
    accessToken: string,
    req: FindAllTopup,
  ): Promise<ApiResponsePaginationTopup> {
    try {
      return await invoke("find_all_topups", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllTopups:", error);
      throw error;
    }
  }

  async findByIdTopup(
    accessToken: string,
    req: FindByIdTopup,
  ): Promise<ApiResponseTopup> {
    try {
      return await invoke("find_by_id_topup", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdTopup:", error);
      throw error;
    }
  }

  async findByActiveTopup(
    accessToken: string,
    req: FindAllTopup,
  ): Promise<ApiResponsePaginationTopupDeleteAt> {
    try {
      return await invoke("find_by_active_topup", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActiveTopup:", error);
      throw error;
    }
  }

  async findByCardNumberTopup(
    accessToken: string,
    req: FindByCardNumberTopup,
  ): Promise<ApiResponsePaginationTopup> {
    try {
      return await invoke("find_by_card_number_topup", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByCardNumberTopup:", error);
      throw error;
    }
  }

  async createTopup(
    accessToken: string,
    req: CreateTopup,
  ): Promise<ApiResponseTopup> {
    try {
      return await invoke("create_topup", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createTopup:", error);
      throw error;
    }
  }

  async updateTopup(
    accessToken: string,
    req: UpdateTopup,
  ): Promise<ApiResponseTopup> {
    try {
      return await invoke("update_topup", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateTopup:", error);
      throw error;
    }
  }

  async trashedTopup(
    accessToken: string,
    req: TrashedTopup,
  ): Promise<ApiResponseTopup> {
    try {
      return await invoke("trashed_topup", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedTopup:", error);
      throw error;
    }
  }
}

export default new TopupCommand();
